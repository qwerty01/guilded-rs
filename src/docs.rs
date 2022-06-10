use std::fmt::Display;
use std::mem;
use std::ops::Deref;
use std::result::Result as StdResult;
use std::str::FromStr;

use async_stream::stream;
use chrono::{DateTime, SecondsFormat, TimeZone, Utc};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use tokio_stream::Stream;

use crate::channel::ChannelId;
use crate::error::Result;
use crate::member::{ServerId, UserId};
use crate::API_BASE;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct DocId(u32);
impl<'de> Deserialize<'de> for DocId {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        u32::deserialize(deserializer).map(Self)
    }
}
impl Serialize for DocId {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl DocId {
    pub fn new(doc: u32) -> Self {
        Self(doc)
    }
}
impl Deref for DocId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Display for DocId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl PartialEq<u32> for DocId {
    fn eq(&self, other: &u32) -> bool {
        &self.0 == other
    }
}
impl PartialEq<str> for DocId {
    fn eq(&self, other: &str) -> bool {
        let other: u32 = match other.parse() {
            Ok(o) => o,
            _ => return false,
        };
        self.0 == other
    }
}
impl FromStr for DocId {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        u32::from_str(s).map(Self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Doc {
    id: DocId,
    #[serde(rename = "serverId")]
    server: ServerId,
    #[serde(rename = "channelId")]
    channel: ChannelId,
    title: String,
    content: String,
    #[serde(rename = "createdAt")]
    created: DateTime<Utc>,
    #[serde(rename = "createdBy")]
    created_by: UserId,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    updated: Option<DateTime<Utc>>,
    #[serde(rename = "updatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_by: Option<UserId>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateDocResponse {
    doc: Doc,
}
#[derive(Debug, Serialize)]
struct CreateDocBody<'a> {
    title: &'a str,
    content: &'a str,
}
impl<'a> CreateDocBody<'a> {
    pub fn new(title: &'a str, content: &'a str) -> Self {
        Self { title, content }
    }
}
#[derive(Debug)]
pub struct CreateDocRequest<'a> {
    client: Client,
    channel: &'a ChannelId,
    title: &'a str,
    content: &'a str,
}
impl<'a> CreateDocRequest<'a> {
    pub fn new(client: Client, channel: &'a ChannelId, title: &'a str, content: &'a str) -> Self {
        Self {
            client,
            channel,
            title,
            content,
        }
    }
    pub async fn send(self) -> Result<Doc> {
        let body = CreateDocBody::new(self.title, self.content);
        let request = self
            .client
            .post(format!("{API_BASE}/channels/{}/docs", self.channel))
            .json(&body)
            .build()?;
        let response = self.client.execute(request).await?.error_for_status()?;
        let doc: CreateDocResponse = response.json().await?;

        Ok(doc.doc)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GetDocsResponse {
    docs: Vec<Doc>,
}
#[derive(Debug)]
enum DocsStream<'a> {
    Uninitialized(GetDocsRequest<'a>),
    Iterating {
        client: Client,
        channel: &'a ChannelId,
        docs: Vec<Doc>,
    },
    Transition,
}
impl<'a> DocsStream<'a> {
    pub fn iter(gdr: GetDocsRequest) -> impl Stream<Item = Result<Doc>> + '_ {
        stream! {
            let mut state = DocsStream::Uninitialized(gdr);

            loop {
                match mem::replace(&mut state, DocsStream::Transition) {
                    DocsStream::Uninitialized(request) => {
                        let client = request.client.clone();
                        let channel = request.channel;
                        let docs = request.send_part().await?;
                        state = DocsStream::Iterating { client, channel, docs };
                        continue;
                    }
                    DocsStream::Iterating {client, channel, docs } => {
                        let mut last_doc = None;
                        for doc in docs {
                            last_doc = Some(doc.created);
                            yield Ok(doc);
                        }
                        if let Some(last_doc) = last_doc {
                            let request = GetDocsRequest::new(client, channel).before(last_doc);
                            state = DocsStream::Uninitialized(request);
                            continue;
                        }
                        break;
                    }
                    DocsStream::Transition => unreachable!("Invariant broken: stream began processing on a state transition"),
                }
            }
        }
    }
}
#[derive(Debug)]
pub struct GetDocsRequest<'a> {
    client: Client,
    channel: &'a ChannelId,
    before: Option<String>,
    limit: Option<u32>,
}
impl<'a> GetDocsRequest<'a> {
    pub fn new(client: Client, channel: &'a ChannelId) -> Self {
        Self {
            client,
            channel,
            before: None,
            limit: None,
        }
    }
    pub fn send(self) -> impl Stream<Item = Result<Doc>> + 'a {
        DocsStream::iter(self)
    }
    async fn send_part(self) -> Result<Vec<Doc>> {
        let mut url: Url = format!("{API_BASE}/channels/{}/docs", self.channel)
            .parse()
            .unwrap();
        if let Some(before) = self.before {
            url.set_query(Some(&format!("before={before}&")));
        }
        if let Some(limit) = self.limit {
            url.set_query(Some(&format!(
                "{}limit={limit}&",
                url.query().unwrap_or_default()
            )))
        }
        let request = self.client.get(url).build()?;
        let response = self.client.execute(request).await?.error_for_status()?;
        let docs: GetDocsResponse = response.json().await?;
        Ok(docs.docs)
    }
    pub fn before<T: TimeZone>(mut self, before: DateTime<T>) -> Self {
        let before = before.with_timezone(&Utc);
        self.before = Some(before.to_rfc3339_opts(SecondsFormat::Millis, true));
        self
    }
    //pub fn limit(mut self, limit: u32) -> Self {
    //    // TODO: Check the limit
    //    self.limit = Some(limit);
    //    self
    //}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GetDocResponse {
    doc: Doc,
}
#[derive(Debug)]
pub struct GetDocRequest<'a> {
    client: Client,
    channel: &'a ChannelId,
    doc: &'a DocId,
}
impl<'a> GetDocRequest<'a> {
    pub fn new(client: Client, channel: &'a ChannelId, doc: &'a DocId) -> Self {
        Self {
            client,
            channel,
            doc,
        }
    }
    pub async fn send(self) -> Result<Doc> {
        let request = self
            .client
            .get(format!(
                "{API_BASE}/channels/{}/docs/{}",
                self.channel, self.doc
            ))
            .build()?;
        let response = self.client.execute(request).await?.error_for_status()?;
        let doc: GetDocResponse = response.json().await?;

        Ok(doc.doc)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateDocResponse {
    doc: Doc,
}
#[derive(Debug, Serialize)]
struct UpdateDocBody<'a> {
    title: &'a str,
    content: &'a str,
}
impl<'a> UpdateDocBody<'a> {
    pub fn new(title: &'a str, content: &'a str) -> Self {
        Self { title, content }
    }
}

#[derive(Debug)]
pub struct UpdateDocRequest<'a> {
    client: Client,
    channel: &'a ChannelId,
    doc: &'a DocId,
    // TODO: optional?
    title: &'a str,
    // TODO: optional?
    content: &'a str,
}
impl<'a> UpdateDocRequest<'a> {
    pub fn new(
        client: Client,
        channel: &'a ChannelId,
        doc: &'a DocId,
        title: &'a str,
        content: &'a str,
    ) -> Self {
        Self {
            client,
            channel,
            doc,
            title,
            content,
        }
    }
    pub async fn send(self) -> Result<Doc> {
        let body = UpdateDocBody::new(self.title, self.content);
        let request = self
            .client
            .put(format!(
                "{API_BASE}/channels/{}/docs/{}",
                self.channel, self.doc
            ))
            .json(&body)
            .build()?;
        let response = self.client.execute(request).await?.error_for_status()?;
        let doc: UpdateDocResponse = response.json().await?;

        Ok(doc.doc)
    }
}

#[derive(Debug)]
pub struct DeleteDocRequest<'a> {
    client: Client,
    channel: &'a ChannelId,
    doc: &'a DocId,
}
impl<'a> DeleteDocRequest<'a> {
    pub fn new(client: Client, channel: &'a ChannelId, doc: &'a DocId) -> Self {
        Self {
            client,
            channel,
            doc,
        }
    }
    pub async fn send(self) -> Result<()> {
        let request = self
            .client
            .delete(format!(
                "{API_BASE}/channels/{}/docs/{}",
                self.channel, self.doc
            ))
            .build()?;
        let _response = self.client.execute(request).await?;
        Ok(())
    }
}
