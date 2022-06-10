use std::fmt::Display;
use std::ops::Deref;
use std::result::Result as StdResult;
use std::str::FromStr;

use crate::groups::GroupId;
use crate::API_BASE;
use crate::{error::Result, member::UserId};
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(transparent)]
// Note: Wrapper type used so that IDs of the same core type cannot be used interchangably
pub struct ChannelId(Uuid);
impl ChannelId {
    pub fn new(channel: Uuid) -> Self {
        Self(channel)
    }
}
impl<'de> Deserialize<'de> for ChannelId {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Uuid::deserialize(deserializer).map(Self)
    }
}
impl Serialize for ChannelId {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl Deref for ChannelId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Display for ChannelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl FromStr for ChannelId {
    type Err = <Uuid as FromStr>::Err;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        Uuid::from_str(s).map(Self)
    }
}
impl PartialEq<str> for ChannelId {
    fn eq(&self, other: &str) -> bool {
        let other: Uuid = match other.parse() {
            Ok(u) => u,
            _ => return false,
        };
        self.0 == other
    }
}
impl PartialEq<Uuid> for ChannelId {
    fn eq(&self, other: &Uuid) -> bool {
        &self.0 == other
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(transparent)]
// Note: Wrapper type used so that IDs of the same core type cannot be used interchangably
pub struct CategoryId(u32);
impl<'de> Deserialize<'de> for CategoryId {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        u32::deserialize(deserializer).map(Self)
    }
}
impl Serialize for CategoryId {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl CategoryId {
    pub fn new(category: u32) -> Self {
        Self(category)
    }
}
impl Deref for CategoryId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Display for CategoryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl PartialEq<u32> for CategoryId {
    fn eq(&self, other: &u32) -> bool {
        &self.0 == other
    }
}
impl PartialEq<str> for CategoryId {
    fn eq(&self, other: &str) -> bool {
        let other: u32 = match other.parse() {
            Ok(o) => o,
            _ => return false,
        };
        self.0 == other
    }
}
impl FromStr for CategoryId {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        u32::from_str(s).map(Self)
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChannelType {
    Announcements,
    Chat,
    Calendar,
    Forums,
    Media,
    Docs,
    Voice,
    List,
    Scheduling,
    Stream,
}

/// Information related to server channels
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServerChannel {
    /// The ID of the channel
    id: ChannelId,
    /// The type of channel.
    /// This will determine what routes to use for creating content in a channel.
    /// For example, if this is "chat", then one must use the routes for creating channel messages
    #[serde(rename = "type")]
    channel_type: ChannelType,
    /// The name of the channel.
    /// (min length 1, max length 100)
    name: String,
    /// The topic of the channel.
    /// (max length 512)
    topic: Option<String>,
    /// The timestamp that the channel was created at
    #[serde(rename = "createdAt")]
    created_at: DateTime<Utc>,
    /// The ID of the user who created this channel
    #[serde(rename = "createdBy")]
    created_by: UserId,
    /// The timestamp that the channel was updated at
    #[serde(rename = "updatedAt")]
    updated: Option<DateTime<Utc>>,
    /// The ID of the server
    #[serde(rename = "serverId")]
    server: String,
    /// The ID of the parent channel or parent thread, if present.
    /// Only relevant for server channels.
    #[serde(rename = "parentId")]
    parent: Option<ChannelId>,
    /// Only relevant for server channels
    #[serde(rename = "categoryId")]
    category: Option<CategoryId>,
    ///
    #[serde(rename = "groupId")]
    group: GroupId,
    /// Whether the channel can be accessed from users who are not members of the server (default: false)
    #[serde(rename = "isPublic")]
    #[serde(default)]
    public: bool,
    /// The ID of the user who archived this channel
    #[serde(rename = "archivedBy")]
    archived_by: Option<UserId>,
    /// The timestamp that the channel was archived at, if relevant
    #[serde(rename = "archivedAt")]
    archived_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ServerChannelResponse {
    channel: ServerChannel,
}

#[derive(Debug, Serialize)]
pub struct CreateChannelRequest<'a> {
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isPublic")]
    public: Option<&'a str>,
    #[serde(rename = "type")]
    channel_type: ChannelType,
    #[serde(rename = "serverId")]
    server: &'a str,
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<&'a GroupId>,
    #[serde(rename = "categoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<&'a CategoryId>,
    #[serde(skip)]
    client: Client,
}

// TODO: ensure set fields follow all requirements from server
impl<'a> CreateChannelRequest<'a> {
    pub fn new(client: Client, server: &'a str, name: &'a str, channel_type: ChannelType) -> Self {
        Self {
            name,
            topic: None,
            public: None,
            channel_type,
            server,
            group: None,
            category: None,
            client,
        }
    }
    pub async fn send(self) -> Result<ServerChannel> {
        let request = self
            .client
            .post(format!("{API_BASE}/channels"))
            .json(&self)
            .build()?;
        let response = self.client.execute(request).await?.error_for_status()?; // TODO: actually make a proper error type
        let channel: ServerChannelResponse = response.json().await?;
        Ok(channel.channel)
    }
    pub fn topic(mut self, topic: &'a str) -> Self {
        self.topic = Some(topic);
        self
    }
    pub fn public(mut self, public: &'a str) -> Self {
        self.public = Some(public);
        self
    }
    pub fn group(mut self, group: &'a GroupId) -> Self {
        self.group = Some(group);
        self
    }
    pub fn category(mut self, category: &'a CategoryId) -> Self {
        self.category = Some(category);
        self
    }
}

#[derive(Debug)]
pub struct GetChannelRequest<'a> {
    client: Client,
    channel: &'a ChannelId,
}
impl<'a> GetChannelRequest<'a> {
    pub fn new(client: Client, channel: &'a ChannelId) -> Self {
        Self { client, channel }
    }
    pub async fn send(self) -> Result<ServerChannel> {
        let request = self
            .client
            .get(format!("{API_BASE}/channels/{}", self.channel))
            .build()?;
        let response = self.client.execute(request).await?.error_for_status()?;
        let channel: ServerChannelResponse = response.json().await?;

        Ok(channel.channel)
    }
}

#[derive(Debug)]
pub struct DeleteChannelRequest<'a> {
    client: Client,
    channel: &'a ChannelId,
}
impl<'a> DeleteChannelRequest<'a> {
    pub fn new(client: Client, channel: &'a ChannelId) -> Self {
        Self { client, channel }
    }
    pub async fn send(self) -> Result<()> {
        let request = self
            .client
            .delete(format!("{API_BASE}/channels/{}", self.channel))
            .build()?;
        self.client.execute(request).await?.error_for_status()?;

        Ok(())
    }
}

pub struct GetChannelsRequest;
