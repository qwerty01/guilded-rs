use std::fmt::Display;
use std::result::Result as StdResult;
use std::str::FromStr;
use std::{mem, ops::Deref};

use crate::channel::ChannelId;
use crate::member::UserId;
use crate::API_BASE;
use async_stream::stream;
use chrono::{DateTime, SecondsFormat, TimeZone, Utc};
use reqwest::{Client, IntoUrl, Url};
use serde::{Deserialize, Serialize};
use tokio_stream::Stream;
use uuid::Uuid;

use crate::error::Result;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(transparent)]
// Note: Wrapper type used so that IDs of the same core type cannot be used interchangably
pub struct MessageId(Uuid);
impl<'de> Deserialize<'de> for MessageId {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Uuid::deserialize(deserializer).map(Self)
    }
}
impl Serialize for MessageId {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl MessageId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
}
impl Deref for MessageId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Display for MessageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl PartialEq<Uuid> for MessageId {
    fn eq(&self, other: &Uuid) -> bool {
        &self.0 == other
    }
}
impl PartialEq<str> for MessageId {
    fn eq(&self, other: &str) -> bool {
        let other: Uuid = match other.parse() {
            Ok(o) => o,
            _ => return false,
        };
        self.0 == other
    }
}
impl FromStr for MessageId {
    type Err = <Uuid as FromStr>::Err;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        Uuid::from_str(s).map(Self)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[repr(transparent)]
// Note: Wrapper type used so that IDs of the same core type cannot be used interchangably
pub struct WebhookId(String);
impl<'de> Deserialize<'de> for WebhookId {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).map(Self)
    }
}
impl Serialize for WebhookId {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl WebhookId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}
impl Deref for WebhookId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Display for WebhookId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl PartialEq<str> for WebhookId {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}
impl FromStr for WebhookId {
    type Err = ();

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        // TODO: validate string
        Ok(Self(s.to_owned()))
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Default,
    System,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    id: MessageId,
    #[serde(rename = "type")]
    message_type: MessageType,
    #[serde(rename = "serverId")]
    server: Option<String>,
    #[serde(rename = "channelId")]
    channel: Option<ChannelId>,
    content: String,
    #[serde(default)]
    embeds: Vec<ChatEmbed>,
    #[serde(default)]
    #[serde(rename = "replyMessageIds")]
    replies: Vec<MessageId>,
    #[serde(default)]
    #[serde(rename = "isPrivate")]
    private: bool,
    created_at: DateTime<Utc>,
    created_by: Option<UserId>,
    #[serde(rename = "createdByWebhookId")]
    webhook: Option<WebhookId>,
    #[serde(rename = "updatedAt")]
    updated: Option<DateTime<Utc>>,
}
impl ChatMessage {
    pub fn id(&self) -> MessageId {
        self.id
    }
    pub fn message_type(&self) -> MessageType {
        self.message_type
    }
    pub fn server(&self) -> Option<&str> {
        self.server.as_ref().map(|v| v as _)
    }
    pub fn channel(&self) -> Option<ChannelId> {
        self.channel
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn embeds(&self) -> &[ChatEmbed] {
        self.embeds.as_slice()
    }
    pub fn replies(&self) -> &[MessageId] {
        self.replies.as_slice()
    }
    pub fn private(&self) -> bool {
        self.private
    }
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    pub fn created_by(&self) -> Option<&UserId> {
        self.created_by.as_ref().map(|v| v as _)
    }
    pub fn webhook(&self) -> Option<&WebhookId> {
        self.webhook.as_ref().map(|v| v as _)
    }
    pub fn updated(&self) -> Option<&DateTime<Utc>> {
        self.updated.as_ref()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatEmbedFooter {
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_url: Option<String>,
    text: String,
}
#[derive(Debug)]
pub struct ChatEmbedFooterBuilder(ChatEmbedFooter);
impl ChatEmbedFooter {
    pub fn new(text: &str) -> Self {
        Self {
            icon_url: None,
            text: text.to_owned(),
        }
    }
    pub fn builder(text: &str) -> ChatEmbedFooterBuilder {
        ChatEmbedFooterBuilder::new(text)
    }
}
impl ChatEmbedFooterBuilder {
    pub fn new(text: &str) -> Self {
        Self(ChatEmbedFooter::new(text))
    }
    pub fn build(self) -> ChatEmbedFooter {
        self.0
    }
    pub fn icon_url(mut self, icon_url: impl IntoUrl) -> Result<Self> {
        self.0.icon_url = Some(icon_url.into_url()?.to_string());
        Ok(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatEmbedThumbnail {
    url: String,
}
impl ChatEmbedThumbnail {
    pub fn new(url: impl IntoUrl) -> Result<Self> {
        Ok(ChatEmbedThumbnail {
            url: url.into_url()?.to_string(),
        })
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatEmbedImage {
    url: String,
}
impl ChatEmbedImage {
    pub fn new(url: impl IntoUrl) -> Result<Self> {
        Ok(Self {
            url: url.into_url()?.to_string(),
        })
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatEmbedAuthor {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_url: Option<String>,
}
#[derive(Debug, Default)]
pub struct ChatEmbedAuthorBuilder(ChatEmbedAuthor);
impl ChatEmbedAuthor {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn builder() -> ChatEmbedAuthorBuilder {
        ChatEmbedAuthorBuilder::new()
    }
}
impl ChatEmbedAuthorBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn build(self) -> ChatEmbedAuthor {
        self.0
    }
    pub fn name(mut self, name: &str) -> Self {
        self.0.name = Some(name.to_owned());
        self
    }
    pub fn url(mut self, url: impl IntoUrl) -> Result<Self> {
        let url = url.into_url()?;
        self.0.url = Some(url.to_string());
        Ok(self)
    }
    pub fn icon_url(mut self, icon_url: impl IntoUrl) -> Result<Self> {
        let icon_url = icon_url.into_url()?;
        self.0.icon_url = Some(icon_url.to_string());
        Ok(self)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChatEmbedField {
    name: String,
    value: String,
    #[serde(default)]
    inline: bool,
}
#[derive(Debug, Default)]
pub struct ChatEmbedFieldBuilder(ChatEmbedField);
impl ChatEmbedField {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_owned(),
            value: value.to_owned(),
            inline: false,
        }
    }
    pub fn builder(name: &str, value: &str) -> ChatEmbedFieldBuilder {
        ChatEmbedFieldBuilder::new(name, value)
    }
}
impl ChatEmbedFieldBuilder {
    pub fn new(name: &str, value: &str) -> Self {
        Self(ChatEmbedField::new(name, value))
    }
    pub fn build(self) -> ChatEmbedField {
        self.0
    }
    pub fn inline(mut self, inline: bool) -> Self {
        self.0.inline = inline;
        self
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ChatEmbed {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    footer: Option<ChatEmbedFooter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail: Option<ChatEmbedThumbnail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<ChatEmbedImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<ChatEmbedAuthor>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    fields: Vec<ChatEmbedField>,
}
impl ChatEmbed {
    pub fn builder() -> ChatEmbedBuilder {
        ChatEmbedBuilder::new()
    }
}

#[derive(Debug, Default)]
pub struct ChatEmbedBuilder(ChatEmbed);
impl ChatEmbedBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn build(self) -> ChatEmbed {
        self.0
    }
    pub fn title(mut self, title: &str) -> Self {
        self.0.title = Some(title.to_owned());
        self
    }
    pub fn description(mut self, description: &str) -> Self {
        self.0.description = Some(description.to_owned());
        self
    }
    pub fn url(mut self, url: impl IntoUrl) -> Result<Self> {
        let url = url.into_url()?;
        self.0.url = Some(url.to_string());
        Ok(self)
    }
    pub fn color(mut self, color: u32) -> Self {
        self.0.color = Some(color);
        self
    }
    pub fn footer(mut self, footer: ChatEmbedFooter) -> Self {
        self.0.footer = Some(footer);
        self
    }
    pub fn timestamp<T: TimeZone>(mut self, timestamp: DateTime<T>) -> Self {
        self.0.timestamp = Some(timestamp.with_timezone(&Utc));
        self
    }
    pub fn thumbnail(mut self, thumbnail: ChatEmbedThumbnail) -> Self {
        self.0.thumbnail = Some(thumbnail);
        self
    }
    pub fn image(mut self, image: ChatEmbedImage) -> Self {
        self.0.image = Some(image);
        self
    }
    pub fn author(mut self, author: ChatEmbedAuthor) -> Self {
        self.0.author = Some(author);
        self
    }
    pub fn add_field(mut self, field: ChatEmbedField) -> Self {
        self.0.fields.push(field);
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateMessageResponse {
    message: ChatMessage,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessageRequest<'a> {
    #[serde(skip)]
    client: Client,
    #[serde(skip)]
    channel_id: &'a ChannelId,
    #[serde(rename = "isPrivate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    private: Option<bool>,
    #[serde(rename = "isSilent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    silent: Option<bool>,
    #[serde(rename = "replyMessageIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    replies: Vec<&'a MessageId>,
    content: &'a str,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    embeds: Vec<ChatEmbed>,
}
impl<'a> CreateMessageRequest<'a> {
    pub fn new(client: Client, channel: &'a ChannelId, content: &'a str) -> Self {
        Self {
            client,
            channel_id: channel,
            private: None,
            silent: None,
            replies: Vec::new(),
            content,
            embeds: Vec::new(),
        }
    }
    pub async fn send(self) -> Result<ChatMessage> {
        let request = self
            .client
            .post(format!("{API_BASE}/channels/{}/messages", self.channel_id))
            .json(&self)
            .build()?;
        let response = self.client.execute(request).await?;
        if let Err(e) = response.error_for_status_ref() {
            println!("Error: {e:?}");
            println!("{}", response.text().await?);
            return Err(e.into());
        }
        let message: CreateMessageResponse = response.json().await?;
        Ok(message.message)
    }
    pub fn private(mut self, private: bool) -> Self {
        self.private = Some(private);
        self
    }
    pub fn silent(mut self, silent: bool) -> Self {
        self.silent = Some(silent);
        self
    }
    pub fn add_reply(mut self, message: &'a MessageId) -> Self {
        self.replies.push(message);
        self
    }
    pub fn add_embed(mut self, embed: ChatEmbed) -> Self {
        self.embeds.push(embed);
        self
    }
}

#[derive(Debug)]
pub struct GetChannelMessagesRequest<'a> {
    client: Client,
    channel: &'a ChannelId,
    before: Option<String>,
    after: Option<String>,
    limit: Option<u32>,
    private: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GetChannelMessagesResponse {
    messages: Vec<ChatMessage>,
}
impl<'a> GetChannelMessagesRequest<'a> {
    pub fn new(client: Client, channel: &'a ChannelId) -> Self {
        Self {
            client,
            channel,
            before: None,
            after: None,
            limit: None,
            private: None,
        }
    }
    pub fn send(self) -> impl Stream<Item = Result<ChatMessage>> + 'a {
        ChannelMessageStream::iter(self)
    }
    async fn send_part(self) -> Result<Vec<ChatMessage>> {
        let mut url: Url = format!("{API_BASE}/channels/{}/messages", self.channel)
            .parse()
            .unwrap();
        if let Some(before) = self.before {
            url.set_query(Some(&format!("before={before}&")));
        }
        if let Some(after) = self.after {
            url.set_query(Some(&format!(
                "{}after={after}&",
                url.query().unwrap_or_default()
            )));
        }
        if let Some(limit) = self.limit {
            url.set_query(Some(&format!(
                "{}limit={limit}&",
                url.query().unwrap_or_default()
            )));
        }
        if let Some(private) = self.private {
            url.set_query(Some(&format!(
                "{}private={private}&",
                url.query().unwrap_or_default()
            )));
        }
        let request = self.client.get(url).build()?;
        let response = self.client.execute(request).await?.error_for_status()?;
        let messages: GetChannelMessagesResponse = response.json().await?;
        Ok(messages.messages)
    }
    pub fn before<T: TimeZone>(mut self, before: DateTime<T>) -> Self {
        let before = before.with_timezone(&Utc);
        self.before = Some(before.to_rfc3339_opts(SecondsFormat::Millis, true));
        self
    }
    pub fn after<T: TimeZone>(mut self, after: DateTime<T>) -> Self {
        let after = after.with_timezone(&Utc);
        self.after = Some(after.to_rfc3339_opts(SecondsFormat::Millis, true));
        self
    }
    //pub fn limit(mut self, limit: u32) -> Self {
    //    // TODO: check the limit
    //    self.limit = Some(limit);
    //    self
    //}
    pub fn private(mut self, private: bool) -> Self {
        self.private = Some(private);
        self
    }
}

enum ChannelMessageStream<'a> {
    Uninitialized(GetChannelMessagesRequest<'a>),
    Iterating {
        client: Client,
        channel: &'a ChannelId,
        after: Option<String>,
        private: Option<bool>,
        messages: Vec<ChatMessage>,
    },
    Transition,
}
impl<'a> ChannelMessageStream<'a> {
    fn iter(request: GetChannelMessagesRequest) -> impl Stream<Item = Result<ChatMessage>> + '_ {
        stream! {
            let mut state = ChannelMessageStream::Uninitialized(request);

            loop {
                match mem::replace(&mut state, ChannelMessageStream::Transition) {
                    ChannelMessageStream::Uninitialized(request) => {
                        let client = request.client.clone();
                        let channel = request.channel;
                        let after = request.after.clone();
                        let private = request.private;
                        let messages = request.send_part().await?;
                        state = ChannelMessageStream::Iterating {
                            client,
                            channel,
                            after,
                            private,
                            messages,
                        };
                        continue
                    },
                    ChannelMessageStream::Iterating {client, channel, after, private, messages} => {
                        let mut last_message = None;
                        for message in messages {
                            last_message = Some(message.created_at);
                            yield Ok(message);
                        }
                        if let Some(last_message) = last_message {
                            let mut request = GetChannelMessagesRequest::new(client, channel).before(last_message);
                            if let Some(after) = after {
                                request = request.after(after.parse::<DateTime<Utc>>().unwrap());
                            }
                            if let Some(private) = private {
                                request = request.private(private);
                            }
                            state = ChannelMessageStream::Uninitialized(request);
                            continue;
                        }
                        break;
                    },
                    ChannelMessageStream::Transition => unreachable!("Invariant broken: stream began processing on a state transition"),
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct GetMessageRequest<'a> {
    client: Client,
    channel: &'a ChannelId,
    message: &'a MessageId,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GetMessageResponse {
    message: ChatMessage,
}
impl<'a> GetMessageRequest<'a> {
    pub fn new(client: Client, channel: &'a ChannelId, message: &'a MessageId) -> Self {
        Self {
            client,
            channel,
            message,
        }
    }
    pub async fn send(self) -> Result<ChatMessage> {
        let url: Url = format!(
            "{API_BASE}/channels/{}/messages/{}",
            self.channel, self.message
        )
        .parse()
        .unwrap();
        let request = self.client.get(url).build()?;
        let response = self.client.execute(request).await?.error_for_status()?;
        let message: GetMessageResponse = response.json().await?;

        Ok(message.message)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct UpdateMessageResponse {
    message: ChatMessage,
}
#[derive(Debug, Serialize, Deserialize)]
struct UpdateMessageRequestBody<'a> {
    content: &'a str,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    embeds: Vec<ChatEmbed>,
}
#[derive(Debug)]
pub struct UpdateMessageRequest<'a> {
    client: Client,
    channel: &'a ChannelId,
    message: &'a MessageId,
    content: UpdateMessageRequestBody<'a>,
}
impl<'a> UpdateMessageRequest<'a> {
    pub fn new(
        client: Client,
        channel: &'a ChannelId,
        message: &'a MessageId,
        content: &'a str,
    ) -> Self {
        Self {
            client,
            channel,
            message,
            content: UpdateMessageRequestBody {
                content,
                embeds: Vec::new(),
            },
        }
    }
    pub async fn send(self) -> Result<ChatMessage> {
        let request = self
            .client
            .put(format!(
                "{API_BASE}/channels/{}/messages/{}",
                self.channel, self.message
            ))
            .json(&self.content)
            .build()?;
        let response = self.client.execute(request).await?.error_for_status()?;
        let message: UpdateMessageResponse = response.json().await?;

        Ok(message.message)
    }
    pub fn add_embed(mut self, embed: ChatEmbed) -> Self {
        self.content.embeds.push(embed);
        self
    }
}

#[derive(Debug)]
pub struct DeleteMessageRequest<'a> {
    client: Client,
    channel: &'a ChannelId,
    message: &'a MessageId,
}
impl<'a> DeleteMessageRequest<'a> {
    pub fn new(client: Client, channel: &'a ChannelId, message: &'a MessageId) -> Self {
        Self {
            client,
            channel,
            message,
        }
    }
    pub async fn send(self) -> Result<()> {
        let request = self
            .client
            .delete(format!(
                "{API_BASE}/channels/{}/messages/{}",
                self.channel, self.message
            ))
            .build()?;
        let _response = self.client.execute(request).await?.error_for_status()?;

        Ok(())
    }
}
