use std::fmt::Display;
use std::ops::Deref;
use std::result::Result as StdResult;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::channel::ChannelId;
use crate::error::Result;
use crate::member::{ServerId, UserId};
use crate::message::WebhookId;
use crate::API_BASE;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(transparent)]
pub struct ForumId(u32);
impl Serialize for ForumId {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for ForumId {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        u32::deserialize(deserializer).map(Self)
    }
}
impl ForumId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}
impl Deref for ForumId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Display for ForumId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl PartialEq<u32> for ForumId {
    fn eq(&self, other: &u32) -> bool {
        &self.0 == other
    }
}
impl PartialEq<str> for ForumId {
    fn eq(&self, other: &str) -> bool {
        let other: u32 = match other.parse() {
            Ok(o) => o,
            _ => return false,
        };
        self.0 == other
    }
}
impl FromStr for ForumId {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        u32::from_str(s).map(Self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForumThread {
    id: ForumId,
    #[serde(rename = "serverId")]
    server: ServerId,
    #[serde(rename = "channelId")]
    channel: ChannelId,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(rename = "createdAt")]
    created: DateTime<Utc>,
    #[serde(rename = "createdBy")]
    created_by: UserId,
    #[serde(rename = "createdByWebhookId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    webhook: Option<WebhookId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "updatedAt")]
    updated: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
struct CreateThreadBody<'a> {
    title: &'a str,
    content: &'a str,
}
impl<'a> CreateThreadBody<'a> {
    pub fn new(title: &'a str, content: &'a str) -> Self {
        Self { title, content }
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateThreadResponse {
    #[serde(rename = "forumThread")]
    thread: ForumThread,
}
#[derive(Debug)]
pub struct CreateThreadRequest<'a> {
    client: Client,
    channel: &'a ChannelId,
    title: &'a str,
    content: &'a str,
}
impl<'a> CreateThreadRequest<'a> {
    pub fn new(client: Client, channel: &'a ChannelId, title: &'a str, content: &'a str) -> Self {
        Self {
            client,
            channel,
            title,
            content,
        }
    }
    pub async fn send(self) -> Result<ForumThread> {
        let body = CreateThreadBody::new(self.title, self.content);
        let request = self
            .client
            .post(format!("{API_BASE}/channels/{}/forum", self.channel))
            .json(&body)
            .build()?;

        let response = self.client.execute(request).await?.error_for_status()?;
        let thread: CreateThreadResponse = response.json().await?;

        Ok(thread.thread)
    }
}
