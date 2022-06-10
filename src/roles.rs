use std::fmt::Display;
use std::ops::Deref;
use std::result::Result as StdResult;
use std::str::FromStr;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::member::{ServerId, UserId};
use crate::API_BASE;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(transparent)]
// Note: Wrapper type used so that IDs of the same core type cannot be used interchangably
pub struct RoleId(u32);
impl<'de> Deserialize<'de> for RoleId {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        u32::deserialize(deserializer).map(Self)
    }
}
impl Serialize for RoleId {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl RoleId {
    pub fn new(role: u32) -> Self {
        Self(role)
    }
}
impl Deref for RoleId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Display for RoleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl PartialEq<u32> for RoleId {
    fn eq(&self, other: &u32) -> bool {
        &self.0 == other
    }
}
impl PartialEq<str> for RoleId {
    fn eq(&self, other: &str) -> bool {
        let other: u32 = match other.parse() {
            Ok(o) => o,
            _ => return false,
        };
        self.0 == other
    }
}
impl FromStr for RoleId {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        u32::from_str(s).map(Self)
    }
}

#[derive(Debug)]
pub struct AssignRoleRequest<'a> {
    client: Client,
    server: &'a ServerId,
    user: &'a UserId,
    role: &'a RoleId,
}
impl<'a> AssignRoleRequest<'a> {
    pub fn new(client: Client, server: &'a ServerId, user: &'a UserId, role: &'a RoleId) -> Self {
        Self {
            client,
            server,
            user,
            role,
        }
    }
    pub async fn send(self) -> Result<()> {
        let request = self
            .client
            .put(format!(
                "{API_BASE}/servers/{}/members/{}/roles/{}",
                self.server, self.user, self.role
            ))
            .build()?;
        let _response = self.client.execute(request).await?.error_for_status()?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct RemoveRoleRequest<'a> {
    client: Client,
    server: &'a ServerId,
    user: &'a UserId,
    role: &'a RoleId,
}
impl<'a> RemoveRoleRequest<'a> {
    pub fn new(client: Client, server: &'a ServerId, user: &'a UserId, role: &'a RoleId) -> Self {
        Self {
            client,
            server,
            user,
            role,
        }
    }
    pub async fn send(self) -> Result<()> {
        let request = self
            .client
            .delete(format!(
                "{API_BASE}/servers/{}/members/{}/roles/{}",
                self.server, self.user, self.role
            ))
            .build()?;
        let _response = self.client.execute(request).await?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct GetMemberRolesResponse {
    #[serde(rename = "roleIds")]
    roles: Vec<RoleId>,
}
#[derive(Debug)]
pub struct GetMemberRolesRequest<'a> {
    client: Client,
    server: &'a ServerId,
    user: &'a UserId,
}
impl<'a> GetMemberRolesRequest<'a> {
    pub fn new(client: Client, server: &'a ServerId, user: &'a UserId) -> Self {
        Self {
            client,
            server,
            user,
        }
    }
    pub async fn send(self) -> Result<Vec<RoleId>> {
        let request = self
            .client
            .get(format!(
                "{API_BASE}/servers/{}/members/{}/roles",
                self.server, self.user
            ))
            .build()?;
        let response = self.client.execute(request).await?.error_for_status()?;
        let roles: GetMemberRolesResponse = response.json().await?;

        Ok(roles.roles)
    }
}
