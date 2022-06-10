use std::fmt::Display;
use std::ops::Deref;
use std::result::Result as StdResult;
use std::str::FromStr;

use async_stream::stream;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio_stream::Stream;
use uuid::Uuid;

use crate::channel::ChannelId;
use crate::error::Result;
use crate::member::{ServerId, UserId};
use crate::message::WebhookId;
use crate::API_BASE;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(transparent)]
pub struct ListId(Uuid);
impl<'de> Deserialize<'de> for ListId {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Uuid::deserialize(deserializer).map(Self)
    }
}
impl Serialize for ListId {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl ListId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
}
impl Deref for ListId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Display for ListId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl FromStr for ListId {
    type Err = <Uuid as FromStr>::Err;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        Uuid::from_str(s).map(Self)
    }
}
impl PartialEq<Uuid> for ListId {
    fn eq(&self, other: &Uuid) -> bool {
        &self.0 == other
    }
}
impl PartialEq<str> for ListId {
    fn eq(&self, other: &str) -> bool {
        let other: Uuid = match other.parse() {
            Ok(o) => o,
            _ => return false,
        };
        self.0 == other
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListItem {
    id: ListId,
    #[serde(rename = "serverId")]
    server: ServerId,
    #[serde(rename = "channelId")]
    channel: ChannelId,
    message: String,
    #[serde(rename = "createdAt")]
    created: DateTime<Utc>,
    #[serde(rename = "createdBy")]
    created_by: UserId,
    #[serde(rename = "createdByWebHook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    webhook: Option<WebhookId>,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    updated: Option<DateTime<Utc>>,
    #[serde(rename = "updatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_by: Option<UserId>,
    #[serde(rename = "parentListItemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<ListId>,
    #[serde(rename = "completedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    completed: Option<DateTime<Utc>>,
    #[serde(rename = "completedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    completed_by: Option<UserId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<ListNote>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListNote {
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
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListItemSummary {
    id: ListId,
    #[serde(rename = "serverId")]
    server: ServerId,
    #[serde(rename = "channelId")]
    channel: ChannelId,
    #[serde(rename = "createdAt")]
    created: DateTime<Utc>,
    #[serde(rename = "createdBy")]
    created_by: UserId,
    #[serde(rename = "createdByWebhookId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    webhook: Option<WebhookId>,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    updated: Option<DateTime<Utc>>,
    #[serde(rename = "updatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_by: Option<UserId>,
    #[serde(rename = "parentListItemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<ListId>,
    #[serde(rename = "completedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    completed: Option<DateTime<Utc>>,
    #[serde(rename = "completedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    completed_by: Option<UserId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<ListNoteSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListNoteSummary {
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

#[derive(Debug, Serialize)]
struct CreateListItemNoteBody<'a> {
    content: &'a str,
}
impl<'a> CreateListItemNoteBody<'a> {
    pub fn new(content: &'a str) -> Self {
        Self { content }
    }
}
#[derive(Debug, Serialize)]
struct CreateListItemBody<'a> {
    message: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<CreateListItemNoteBody<'a>>,
}
impl<'a> CreateListItemBody<'a> {
    pub fn new(message: &'a str) -> Self {
        Self {
            message,
            note: None,
        }
    }
    pub fn note(&mut self, note: &'a str) {
        self.note = Some(CreateListItemNoteBody::new(note))
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct CreateListItemResponse {
    #[serde(rename = "listItem")]
    item: ListItem,
}
#[derive(Debug)]
pub struct CreateListItemRequest<'a> {
    client: Client,
    channel: &'a ChannelId,
    message: &'a str,
    note: Option<&'a str>,
}
impl<'a> CreateListItemRequest<'a> {
    pub fn new(client: Client, channel: &'a ChannelId, message: &'a str) -> Self {
        Self {
            client,
            channel,
            message,
            note: None,
        }
    }
    pub async fn send(self) -> Result<ListItem> {
        let mut body = CreateListItemBody::new(self.message);
        if let Some(note) = self.note {
            body.note(note);
        }
        let request = self
            .client
            .post(format!("{API_BASE}/channels/{}/items", self.channel))
            .json(&body)
            .build()?;
        let response = self.client.execute(request).await?.error_for_status()?;
        let item: CreateListItemResponse = response.json().await?;
        Ok(item.item)
    }
    pub fn note(mut self, note: &'a str) -> Self {
        self.note = Some(note);
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GetListItemsResponse {
    #[serde(rename = "listItems")]
    items: Vec<ListItemSummary>,
}
#[derive(Debug)]
struct ListItemsStream;
impl ListItemsStream {
    fn iter(glir: GetListItemsRequest) -> impl Stream<Item = Result<ListItemSummary>> + '_ {
        stream! {
            let request = glir.client.get(format!("{API_BASE}/channels/{}/items", glir.channel)).build()?;
            let response = glir.client.execute(request).await?.error_for_status()?;
            let items: GetListItemsResponse = response.json().await?;

            for item in items.items {
                yield Ok(item)
            }
        }
    }
}
#[derive(Debug)]
pub struct GetListItemsRequest<'a> {
    client: Client,
    channel: &'a ChannelId,
}
impl<'a> GetListItemsRequest<'a> {
    pub fn new(client: Client, channel: &'a ChannelId) -> Self {
        Self { client, channel }
    }
    pub fn send(self) -> impl Stream<Item = Result<ListItemSummary>> + 'a {
        ListItemsStream::iter(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GetListItemResponse {
    #[serde(rename = "listItem")]
    item: ListItem,
}
#[derive(Debug)]
pub struct GetListItemRequest<'a> {
    client: Client,
    channel: &'a ChannelId,
    item: &'a ListId,
}
impl<'a> GetListItemRequest<'a> {
    pub fn new(client: Client, channel: &'a ChannelId, item: &'a ListId) -> Self {
        Self {
            client,
            channel,
            item,
        }
    }
    pub async fn send(self) -> Result<ListItem> {
        let request = self
            .client
            .get(format!(
                "{API_BASE}/channels/{}/items/{}",
                self.channel, self.item
            ))
            .build()?;
        let response = self.client.execute(request).await?.error_for_status()?;
        let item: GetListItemResponse = response.json().await?;

        Ok(item.item)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct UpdateListItemResponse {
    #[serde(rename = "listItem")]
    item: ListItem,
}
#[derive(Debug, Serialize)]
struct UpdateListItemNote<'a> {
    content: &'a str,
}
impl<'a> UpdateListItemNote<'a> {
    pub fn new(content: &'a str) -> Self {
        Self { content }
    }
}
#[derive(Debug, Serialize)]
struct UpdateListItemBody<'a> {
    message: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<UpdateListItemNote<'a>>,
}
impl<'a> UpdateListItemBody<'a> {
    pub fn new(message: &'a str) -> Self {
        Self {
            message,
            note: None,
        }
    }
    pub fn note(&mut self, note: &'a str) {
        self.note = Some(UpdateListItemNote::new(note));
    }
}
#[derive(Debug)]
pub struct UpdateListItemRequest<'a> {
    client: Client,
    channel: &'a ChannelId,
    item: &'a ListId,
    message: &'a str,
    note: Option<&'a str>,
}
impl<'a> UpdateListItemRequest<'a> {
    pub fn new(client: Client, channel: &'a ChannelId, item: &'a ListId, message: &'a str) -> Self {
        Self {
            client,
            channel,
            item,
            message,
            note: None,
        }
    }
    pub async fn send(self) -> Result<ListItem> {
        let mut body = UpdateListItemBody::new(self.message);
        if let Some(note) = self.note {
            body.note(note);
        }
        let request = self
            .client
            .put(format!(
                "{API_BASE}/channels/{}/items/{}",
                self.channel, self.item
            ))
            .json(&body)
            .build()?;
        let response = self.client.execute(request).await?.error_for_status()?;
        let item: UpdateListItemResponse = response.json().await?;

        Ok(item.item)
    }
    pub fn note(mut self, note: &'a str) -> Self {
        self.note = Some(note);
        self
    }
}

#[derive(Debug)]
pub struct DeleteListItemRequest<'a> {
    client: Client,
    channel: &'a ChannelId,
    item: &'a ListId,
}
impl<'a> DeleteListItemRequest<'a> {
    pub fn new(client: Client, channel: &'a ChannelId, item: &'a ListId) -> Self {
        Self {
            client,
            channel,
            item,
        }
    }
    pub async fn send(self) -> Result<()> {
        let request = self
            .client
            .delete(format!(
                "{API_BASE}/channels/{}/items/{}",
                self.channel, self.item
            ))
            .build()?;
        let _response = self.client.execute(request).await?.error_for_status()?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct CompleteListItemRequest<'a> {
    client: Client,
    channel: &'a ChannelId,
    item: &'a ListId,
}
impl<'a> CompleteListItemRequest<'a> {
    pub fn new(client: Client, channel: &'a ChannelId, item: &'a ListId) -> Self {
        Self {
            client,
            channel,
            item,
        }
    }
    pub async fn send(self) -> Result<()> {
        let request = self
            .client
            .post(format!(
                "{API_BASE}/channels/{}/items/{}/complete",
                self.channel, self.item
            ))
            .build()?;
        let _response = self.client.execute(request).await?.error_for_status()?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct UncompleteListItemRequest<'a> {
    client: Client,
    channel: &'a ChannelId,
    item: &'a ListId,
}
impl<'a> UncompleteListItemRequest<'a> {
    pub fn new(client: Client, channel: &'a ChannelId, item: &'a ListId) -> Self {
        Self {
            client,
            channel,
            item,
        }
    }
    pub async fn send(self) -> Result<()> {
        let request = self
            .client
            .delete(format!(
                "{API_BASE}/channels/{}/items/{}/complete",
                self.channel, self.item
            ))
            .build()?;
        let _response = self.client.execute(request).await?.error_for_status()?;

        Ok(())
    }
}
