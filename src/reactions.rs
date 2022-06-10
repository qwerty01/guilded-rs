use std::fmt::Display;
use std::ops::Deref;
use std::result::Result as StdResult;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::channel::ChannelId;
use crate::docs::DocId;
use crate::error::Result;
use crate::forums::ForumId;
use crate::list::ListId;
use crate::member::{ServerId, UserId};
use crate::message::{MessageId, WebhookId};
use crate::API_BASE;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(transparent)]
pub struct EmoteId(u32);
impl<'de> Deserialize<'de> for EmoteId {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        u32::deserialize(deserializer).map(Self)
    }
}
impl Serialize for EmoteId {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl EmoteId {
    pub fn new(reaction: u32) -> Self {
        Self(reaction)
    }
}
impl Deref for EmoteId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Display for EmoteId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl PartialEq<u32> for EmoteId {
    fn eq(&self, other: &u32) -> bool {
        &self.0 == other
    }
}
impl PartialEq<str> for EmoteId {
    fn eq(&self, other: &str) -> bool {
        let other: u32 = match other.parse() {
            Ok(o) => o,
            _ => return false,
        };
        self.0 == other
    }
}
impl FromStr for EmoteId {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        u32::from_str(s).map(Self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    id: EmoteId,
    #[serde(rename = "serverId")]
    server: Option<ServerId>,
    #[serde(rename = "createdAt")]
    created: DateTime<Utc>,
    #[serde(rename = "createdBy")]
    created_by: UserId,
    #[serde(rename = "createdByWebhookId")]
    webhook: Option<WebhookId>,
}

#[derive(Debug)]
pub enum ContentId<'a> {
    Channel(&'a ChannelId),
    Doc(&'a DocId),
    Forum(&'a ForumId),
    List(&'a ListId),
    Message(&'a MessageId),
}
impl<'a> Serialize for ContentId<'a> {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ContentId::Channel(channel) => channel.serialize(serializer),
            ContentId::Doc(doc) => doc.serialize(serializer),
            ContentId::Forum(forum) => forum.serialize(serializer),
            ContentId::List(list) => list.serialize(serializer),
            ContentId::Message(message) => message.serialize(serializer),
        }
    }
}
impl<'a> From<&'a ChannelId> for ContentId<'a> {
    fn from(channel: &'a ChannelId) -> Self {
        Self::Channel(channel)
    }
}
impl<'a> From<&'a DocId> for ContentId<'a> {
    fn from(doc: &'a DocId) -> Self {
        Self::Doc(doc)
    }
}
impl<'a> From<&'a ForumId> for ContentId<'a> {
    fn from(forum: &'a ForumId) -> Self {
        Self::Forum(forum)
    }
}
impl<'a> From<&'a ListId> for ContentId<'a> {
    fn from(list: &'a ListId) -> Self {
        ContentId::List(list)
    }
}
impl<'a> From<&'a MessageId> for ContentId<'a> {
    fn from(message: &'a MessageId) -> Self {
        Self::Message(message)
    }
}
impl<'a> Display for ContentId<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Channel(channel) => channel.fmt(f),
            Self::Doc(doc) => doc.fmt(f),
            Self::Forum(forum) => forum.fmt(f),
            Self::List(list) => list.fmt(f),
            Self::Message(message) => message.fmt(f),
        }
    }
}

#[derive(Debug)]
pub struct AddReactionRequest<'a> {
    client: Client,
    channel: &'a ChannelId,
    content: ContentId<'a>,
    emote: &'a EmoteId,
}
impl<'a> AddReactionRequest<'a> {
    pub fn new<C: Into<ContentId<'a>>>(
        client: Client,
        channel: &'a ChannelId,
        content: C,
        emote: &'a EmoteId,
    ) -> Self {
        Self {
            client,
            channel,
            content: content.into(),
            emote,
        }
    }
    pub async fn send(self) -> Result<()> {
        let request = self
            .client
            .put(format!(
                "{API_BASE}/channels/{}/content/{}/emotes/{}",
                self.channel, self.content, self.emote
            ))
            .build()?;
        let _response = self.client.execute(request).await?.error_for_status()?;

        Ok(())
    }
}
