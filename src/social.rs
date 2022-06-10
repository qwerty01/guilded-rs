use std::fmt::Display;
use std::result::Result as StdResult;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::member::{ServerId, UserId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SocialMediaType {
    Roblox,
    Twitch,
    #[serde(rename = "bnet")]
    Blizzard,
    Steam,
    Xbox,
    PSN,
    Origin,
    #[serde(rename = "switch")]
    Nintendo,
    Twitter,
    YouTube,
    Patreon,
}
impl SocialMediaType {
    pub fn name(&self) -> &'static str {
        match self {
            SocialMediaType::Roblox => "roblox",
            SocialMediaType::Twitch => "twitch",
            SocialMediaType::Blizzard => "bnet",
            SocialMediaType::Steam => "steam",
            SocialMediaType::Xbox => "xbox",
            SocialMediaType::PSN => "psn",
            SocialMediaType::Origin => "origin",
            SocialMediaType::Nintendo => "switch",
            SocialMediaType::Twitter => "twitter",
            SocialMediaType::YouTube => "youtube",
            SocialMediaType::Patreon => "patreon",
        }
    }
}
impl Display for SocialMediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
#[derive(Debug)]
pub struct GetSocialLinksRequest<'a> {
    client: Client,
    server: &'a ServerId,
    user: &'a UserId,
    link_type: SocialMediaType,
}
impl<'a> GetSocialLinksRequest<'a> {
    pub fn new(
        client: Client,
        server: &'a ServerId,
        user: &'a UserId,
        link_type: SocialMediaType,
    ) -> Self {
        Self {
            client,
            server,
            user,
            link_type,
        }
    }
}
