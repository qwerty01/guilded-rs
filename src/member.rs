use std::result::Result as StdResult;
use std::str::FromStr;
use std::{collections::HashSet, fmt::Display, ops::Deref};

use async_stream::stream;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio_stream::Stream;

use crate::error::Result;
use crate::roles::RoleId;
use crate::API_BASE;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[repr(transparent)]
// Note: Wrapper type used so that IDs of the same core type cannot be used interchangably
pub struct UserId(String);
impl<'de> Deserialize<'de> for UserId {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).map(Self)
    }
}
impl Serialize for UserId {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl UserId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}
impl Deref for UserId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl PartialEq<str> for UserId {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}
impl FromStr for UserId {
    type Err = ();

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        // TODO: validate the string
        Ok(Self(s.to_owned()))
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[repr(transparent)]
// Note: Wrapper type used so that IDs of the same core type cannot be used interchangably
pub struct ServerId(String);
impl<'de> Deserialize<'de> for ServerId {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).map(Self)
    }
}
impl Serialize for ServerId {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl ServerId {
    pub fn new(server: String) -> Self {
        Self(server)
    }
}
impl Deref for ServerId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Display for ServerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl PartialEq<str> for ServerId {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}
impl FromStr for ServerId {
    type Err = ();

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        // TODO: validate the string
        Ok(Self(s.to_owned()))
    }
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserType {
    /// The user is a bot
    Bot,
    /// The user is a human
    User,
}

fn default_usertype() -> UserType {
    UserType::User
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct User {
    /// ID of the user
    id: UserId,
    /// Type of user
    #[serde(default = "default_usertype")]
    user_type: UserType,
    /// Name of user
    name: String,
    /// Avatar image of user
    avatar: Option<String>,
    /// Banner image of user
    banner: Option<String>,
    /// Timestamp of when the user was created
    #[serde(rename = "createdAt")]
    created: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServerMember {
    /// User associated with member
    user: User,
    /// Set of roles assigned to user
    #[serde(rename = "roleIds")]
    roles: HashSet<RoleId>,
    /// User's server nickname
    nickname: Option<String>,
    /// Timestamp of when the user joined the server
    #[serde(rename = "joinedAt")]
    joined: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserSummary {
    /// ID of the user
    id: UserId,
    /// Type of user
    #[serde(default = "default_usertype")]
    #[serde(rename = "type")]
    user_type: UserType,
    /// Name of user
    name: String,
    /// Avatar image of user
    avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServerMemberSummary {
    /// User associated with member
    user: UserSummary,
    /// Set of roles assigned to user
    #[serde(rename = "roleIds")]
    roles: HashSet<RoleId>,
}

#[derive(Debug, Serialize)]
struct UpdateNicknameRequestData<'a> {
    nickname: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct UpdateNicknameResponse {
    nickname: String,
}

#[derive(Debug)]
pub struct UpdateNicknameRequest<'a> {
    client: Client,
    server: &'a ServerId,
    user: &'a UserId,
    nickname: UpdateNicknameRequestData<'a>,
}
impl<'a> UpdateNicknameRequest<'a> {
    pub fn new(client: Client, server: &'a ServerId, user: &'a UserId, nickname: &'a str) -> Self {
        Self {
            client,
            server,
            user,
            nickname: UpdateNicknameRequestData { nickname },
        }
    }
    pub async fn send(self) -> Result<String> {
        // TODO: sanitize server/user
        let request = self
            .client
            .put(format!(
                "{API_BASE}/servers/{}/members/{}/nickname",
                self.server, self.user
            ))
            .json(&self.nickname)
            .build()?;
        let response = self.client.execute(request).await?.error_for_status()?;
        let nickname: UpdateNicknameResponse = response.json().await?;

        Ok(nickname.nickname)
    }
}

#[derive(Debug)]
pub struct DeleteNicknameRequest<'a> {
    client: Client,
    server: &'a ServerId,
    user: &'a UserId,
}
impl<'a> DeleteNicknameRequest<'a> {
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
                "{API_BASE}/servers/{}/members/{}/nickname",
                self.server, self.user
            ))
            .build()?;
        let _response = self.client.execute(request).await?.error_for_status()?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GetMemberResponse {
    member: ServerMember,
}
#[derive(Debug)]
pub struct GetMemberRequest<'a> {
    client: Client,
    server: &'a ServerId,
    user: &'a UserId,
}
impl<'a> GetMemberRequest<'a> {
    pub fn new(client: Client, server: &'a ServerId, user: &'a UserId) -> Self {
        Self {
            client,
            server,
            user,
        }
    }
    pub async fn send(self) -> Result<ServerMember> {
        let request = self
            .client
            .get(format!(
                "{API_BASE}/servers/{}/members/{}",
                self.server, self.user
            ))
            .build()?;
        let response = self.client.execute(request).await?.error_for_status()?;
        let member: GetMemberResponse = response.json().await?;
        Ok(member.member)
    }
}

#[derive(Debug)]
pub struct KickMemberRequest<'a> {
    client: Client,
    server: &'a ServerId,
    user: &'a UserId,
}
impl<'a> KickMemberRequest<'a> {
    pub fn new(client: Client, server: &'a ServerId, user: &'a UserId) -> Self {
        KickMemberRequest {
            client,
            server,
            user,
        }
    }
    pub async fn send(self) -> Result<()> {
        let request = self
            .client
            .delete(format!(
                "{API_BASE}/servers/{}/members/{}",
                self.server, self.user
            ))
            .build()?;
        let _response = self.client.execute(request).await?.error_for_status()?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GetMembersResponse {
    members: Vec<ServerMemberSummary>,
}
#[derive(Debug)]
struct MemberStream;
impl MemberStream {
    fn iter(gmr: GetMembersRequest) -> impl Stream<Item = Result<ServerMemberSummary>> + '_ {
        stream! {
            let request = gmr
                .client
                .get(format!("{API_BASE}/servers/{}/members", gmr.server))
                .build()?;
            let response = gmr.client.execute(request).await?.error_for_status()?;
            let members: GetMembersResponse = response.json().await?;
            for member in members.members {
                yield Ok(member);
            }
        }
    }
}
#[derive(Debug)]
pub struct GetMembersRequest<'a> {
    client: Client,
    server: &'a ServerId,
}
impl<'a> GetMembersRequest<'a> {
    pub fn new(client: Client, server: &'a ServerId) -> Self {
        Self { client, server }
    }
    pub fn send(self) -> impl Stream<Item = Result<ServerMemberSummary>> + 'a {
        MemberStream::iter(self)
    }
}
