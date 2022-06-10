use std::fmt::Display;
use std::ops::Deref;
use std::result::Result as StdResult;
use std::str::FromStr;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::member::UserId;
use crate::API_BASE;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[repr(transparent)]
// Note: Wrapper type used so that IDs of the same core type cannot be used interchangably
pub struct GroupId(String);
impl<'de> Deserialize<'de> for GroupId {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).map(Self)
    }
}
impl Serialize for GroupId {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl GroupId {
    pub fn new(group: String) -> Self {
        Self(group)
    }
}
impl Deref for GroupId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Display for GroupId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl PartialEq<str> for GroupId {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}
impl FromStr for GroupId {
    type Err = ();

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        // TODO: validate the string
        Ok(Self(s.to_owned()))
    }
}

#[derive(Debug)]
pub struct AddGroupMemberRequest<'a> {
    client: Client,
    group: &'a GroupId,
    user: &'a UserId,
}
impl<'a> AddGroupMemberRequest<'a> {
    pub fn new(client: Client, group: &'a GroupId, user: &'a UserId) -> Self {
        Self {
            client,
            group,
            user,
        }
    }
    pub async fn send(self) -> Result<()> {
        let request = self
            .client
            .put(format!(
                "{API_BASE}/groups/{}/members/{}",
                self.group, self.user
            ))
            .build()?;
        let _response = self.client.execute(request).await?.error_for_status()?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct DeleteGroupMemberRequest<'a> {
    client: Client,
    group: &'a GroupId,
    user: &'a UserId,
}
impl<'a> DeleteGroupMemberRequest<'a> {
    pub fn new(client: Client, group: &'a GroupId, user: &'a UserId) -> Self {
        Self {
            client,
            group,
            user,
        }
    }
    pub async fn send(self) -> Result<()> {
        let request = self
            .client
            .delete(format!(
                "{API_BASE}/groups/{}/members/{}",
                self.group, self.user
            ))
            .build()?;
        let _response = self.client.execute(request).await?.error_for_status()?;

        Ok(())
    }
}
