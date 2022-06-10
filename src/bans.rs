use async_stream::stream;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio_stream::Stream;

use crate::error::Result;
use crate::member::{ServerId, UserId, UserSummary};
use crate::API_BASE;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ServerMemberBan {
    user: UserSummary,
    reason: Option<String>,
    created_by: UserId,
    #[serde(rename = "createdAt")]
    created: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ServerBanResponse {
    #[serde(rename = "serverMemberBan")]
    ban: ServerMemberBan,
}
#[derive(Debug, Default, Serialize)]
struct ServerBanBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<&'a str>,
}
impl<'a> ServerBanBody<'a> {
    pub fn new(reason: Option<&'a str>) -> Self {
        Self { reason }
    }
}
#[derive(Debug)]
pub struct ServerBanRequest<'a> {
    client: Client,
    server: &'a ServerId,
    user: &'a UserId,
    reason: Option<&'a str>,
}
impl<'a> ServerBanRequest<'a> {
    pub fn new(client: Client, server: &'a ServerId, user: &'a UserId) -> Self {
        Self {
            client,
            server,
            user,
            reason: None,
        }
    }
    pub async fn send(self) -> Result<ServerMemberBan> {
        let body = ServerBanBody::new(self.reason);
        let request = self
            .client
            .post(format!(
                "{API_BASE}/servers/{}/bans/{}",
                self.server, self.user
            ))
            .json(&body)
            .build()?;
        let response = self.client.execute(request).await?.error_for_status()?;
        let ban: ServerBanResponse = response.json().await?;

        Ok(ban.ban)
    }
    pub fn reason(mut self, reason: &'a str) -> Self {
        self.reason = Some(reason);
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct GetServerBanResponse {
    #[serde(rename = "serverMemberBan")]
    ban: ServerMemberBan,
}
#[derive(Debug)]
pub struct GetServerBanRequest<'a> {
    client: Client,
    server: &'a ServerId,
    user: &'a UserId,
}
impl<'a> GetServerBanRequest<'a> {
    pub fn new(client: Client, server: &'a ServerId, user: &'a UserId) -> Self {
        Self {
            client,
            server,
            user,
        }
    }
    // TODO: change to option
    pub async fn send(self) -> Result<ServerMemberBan> {
        let request = self
            .client
            .get(format!(
                "{API_BASE}/servers/{}/bans/{}",
                self.server, self.user
            ))
            .build()?;
        let response = self.client.execute(request).await?.error_for_status()?;
        let ban: GetServerBanResponse = response.json().await?;

        Ok(ban.ban)
    }
}

#[derive(Debug)]
pub struct DeleteServerBanRequest<'a> {
    client: Client,
    server: &'a ServerId,
    user: &'a UserId,
}
impl<'a> DeleteServerBanRequest<'a> {
    pub fn new(client: Client, server: &'a ServerId, user: &'a UserId) -> Self {
        Self {
            client,
            server,
            user,
        }
    }
    pub async fn send(self) -> Result<()> {
        let request = self
            .client
            .delete(format!(
                "{API_BASE}/servers/{}/bans/{}",
                self.server, self.user
            ))
            .build()?;
        let _response = self.client.execute(request).await?.error_for_status()?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct GetServerBansResponse {
    #[serde(rename = "serverMemberBans")]
    bans: Vec<ServerMemberBan>,
}

#[derive(Debug)]
struct GetServerBansStream;
impl GetServerBansStream {
    fn iter(gsbr: GetServerBansRequest) -> impl Stream<Item = Result<ServerMemberBan>> + '_ {
        stream! {
            let request = gsbr.client.get(format!("{API_BASE}/servers/{}/bans", gsbr.server)).build()?;
            let response = gsbr.client.execute(request).await?.error_for_status()?;
            let bans: GetServerBansResponse = response.json().await?;

            for ban in bans.bans {
                yield Ok(ban)
            }
        }
    }
}
#[derive(Debug)]
pub struct GetServerBansRequest<'a> {
    client: Client,
    server: &'a ServerId,
}
impl<'a> GetServerBansRequest<'a> {
    pub fn new(client: Client, server: &'a ServerId) -> Self {
        Self { client, server }
    }
    pub fn send(self) -> impl Stream<Item = Result<ServerMemberBan>> + 'a {
        GetServerBansStream::iter(self)
    }
}
