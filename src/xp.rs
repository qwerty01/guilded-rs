use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::member::{ServerId, UserId};
use crate::roles::RoleId;
use crate::API_BASE;

#[derive(Debug, Serialize, Deserialize)]
struct MemberXpResponse {
    total: i32,
}
#[derive(Debug, Serialize)]
struct MemberXpBody {
    amount: i32,
}
impl MemberXpBody {
    pub fn new(amount: i32) -> Self {
        Self { amount }
    }
}
#[derive(Debug)]
pub struct MemberXpRequest<'a> {
    client: Client,
    server: &'a ServerId,
    user: &'a UserId,
    amount: i32,
}
impl<'a> MemberXpRequest<'a> {
    pub fn new(client: Client, server: &'a ServerId, user: &'a UserId, amount: i32) -> Self {
        Self {
            client,
            server,
            user,
            amount,
        }
    }
    pub async fn send(self) -> Result<i32> {
        let body = MemberXpBody::new(self.amount);
        let request = self
            .client
            .post(format!(
                "{API_BASE}/servers/{}/members/{}/xp",
                self.server, self.user
            ))
            .json(&body)
            .build()?;
        let response = self.client.execute(request).await?.error_for_status()?;
        let total: MemberXpResponse = response.json().await?;

        Ok(total.total)
    }
}

#[derive(Debug, Serialize)]
struct RoleXpBody {
    amount: i32,
}
impl RoleXpBody {
    pub fn new(amount: i32) -> Self {
        Self { amount }
    }
}
#[derive(Debug)]
pub struct RoleXpRequest<'a> {
    client: Client,
    server: &'a ServerId,
    role: &'a RoleId,
    amount: i32,
}
impl<'a> RoleXpRequest<'a> {
    pub fn new(client: Client, server: &'a ServerId, role: &'a RoleId, amount: i32) -> Self {
        Self {
            client,
            server,
            role,
            amount,
        }
    }
    pub async fn send(self) -> Result<()> {
        let body = RoleXpBody::new(self.amount);
        let request = self
            .client
            .post(format!(
                "{API_BASE}/servers/{}/roles/{}/xp",
                self.server, self.role
            ))
            .json(&body)
            .build()?;
        let _response = self.client.execute(request).await?.error_for_status()?;

        Ok(())
    }
}
