use bans::{DeleteServerBanRequest, GetServerBanRequest, GetServerBansRequest, ServerBanRequest};
use channel::{
    ChannelId, ChannelType, CreateChannelRequest, DeleteChannelRequest, GetChannelRequest,
};
use docs::{
    CreateDocRequest, DeleteDocRequest, DocId, GetDocRequest, GetDocsRequest, UpdateDocRequest,
};
use forums::CreateThreadRequest;
use groups::{AddGroupMemberRequest, DeleteGroupMemberRequest, GroupId};
use list::{
    CompleteListItemRequest, CreateListItemRequest, DeleteListItemRequest, GetListItemRequest,
    GetListItemsRequest, ListId, UncompleteListItemRequest, UpdateListItemRequest,
};
use member::{
    DeleteNicknameRequest, GetMemberRequest, GetMembersRequest, KickMemberRequest, ServerId,
    UpdateNicknameRequest, UserId,
};
use message::{
    CreateMessageRequest, DeleteMessageRequest, GetChannelMessagesRequest, GetMessageRequest,
    MessageId, UpdateMessageRequest,
};
use reactions::{AddReactionRequest, ContentId, EmoteId};
use reqwest::header::{self, HeaderMap, InvalidHeaderValue};
use reqwest::Client;
use roles::{GetMemberRolesRequest, RoleId};
use std::ops::Deref;
use xp::{MemberXpRequest, RoleXpRequest};

pub mod bans;
pub mod channel;
pub mod docs;
pub mod error;
pub mod forums;
pub mod groups;
pub mod list;
pub mod member;
pub mod message;
pub mod reactions;
pub mod roles;
pub mod social;
pub mod xp;

static API_BASE: &str = "https://www.guilded.gg/api/v1";

#[derive(Debug, Clone)]
pub struct GuildedClient(Client);
impl GuildedClient {
    pub fn new(token: &str) -> Result<Self, InvalidHeaderValue> {
        let mut hm = HeaderMap::new();
        hm.insert(header::AUTHORIZATION, format!("Bearer {token}").parse()?);
        let client = Client::builder().default_headers(hm).build().unwrap();
        Ok(Self(client))
    }
    pub fn create_channel<'a>(
        &self,
        server: &'a str,
        name: &'a str,
        channel_type: ChannelType,
    ) -> CreateChannelRequest<'a> {
        CreateChannelRequest::new(self.0.clone(), server, name, channel_type)
    }
    pub fn get_channel<'a>(&self, id: &'a ChannelId) -> GetChannelRequest<'a> {
        GetChannelRequest::new(self.0.clone(), id)
    }
    pub fn delete_channel<'a>(&self, id: &'a ChannelId) -> DeleteChannelRequest<'a> {
        DeleteChannelRequest::new(self.0.clone(), id)
    }
    pub fn get_channels(&self) -> GetChannelRequest {
        unimplemented!()
    }
    pub fn send_message<'a>(
        &self,
        channel: &'a ChannelId,
        content: &'a str,
    ) -> CreateMessageRequest<'a> {
        CreateMessageRequest::new(self.0.clone(), channel, content)
    }
    pub fn get_messages<'a>(&self, channel: &'a ChannelId) -> GetChannelMessagesRequest<'a> {
        GetChannelMessagesRequest::new(self.0.clone(), channel)
    }
    pub fn get_message<'a>(
        &self,
        channel: &'a ChannelId,
        message: &'a MessageId,
    ) -> GetMessageRequest<'a> {
        GetMessageRequest::new(self.0.clone(), channel, message)
    }
    pub fn update_message<'a>(
        &self,
        channel: &'a ChannelId,
        message: &'a MessageId,
        content: &'a str,
    ) -> UpdateMessageRequest<'a> {
        UpdateMessageRequest::new(self.0.clone(), channel, message, content)
    }
    pub fn delete_message<'a>(
        &self,
        channel: &'a ChannelId,
        message: &'a MessageId,
    ) -> DeleteMessageRequest<'a> {
        DeleteMessageRequest::new(self.0.clone(), channel, message)
    }
    pub fn update_nickname<'a>(
        &self,
        server: &'a ServerId,
        user: &'a UserId,
        nickname: &'a str,
    ) -> UpdateNicknameRequest<'a> {
        UpdateNicknameRequest::new(self.0.clone(), server, user, nickname)
    }
    pub fn delete_nickname<'a>(
        &self,
        server: &'a ServerId,
        user: &'a UserId,
    ) -> DeleteNicknameRequest<'a> {
        DeleteNicknameRequest::new(self.0.clone(), server, user)
    }
    pub fn get_member<'a>(&self, server: &'a ServerId, user: &'a UserId) -> GetMemberRequest<'a> {
        GetMemberRequest::new(self.0.clone(), server, user)
    }
    pub fn kick_member<'a>(&self, server: &'a ServerId, user: &'a UserId) -> KickMemberRequest<'a> {
        KickMemberRequest::new(self.0.clone(), server, user)
    }
    pub fn get_members<'a>(&self, server: &'a ServerId) -> GetMembersRequest<'a> {
        GetMembersRequest::new(self.0.clone(), server)
    }
    pub fn ban_user<'a>(&self, server: &'a ServerId, user: &'a UserId) -> ServerBanRequest<'a> {
        ServerBanRequest::new(self.0.clone(), server, user)
    }
    pub fn get_ban<'a>(&self, server: &'a ServerId, user: &'a UserId) -> GetServerBanRequest<'a> {
        GetServerBanRequest::new(self.0.clone(), server, user)
    }
    pub fn delete_ban<'a>(
        &self,
        server: &'a ServerId,
        user: &'a UserId,
    ) -> DeleteServerBanRequest<'a> {
        DeleteServerBanRequest::new(self.0.clone(), server, user)
    }
    pub fn get_bans<'a>(&self, server: &'a ServerId) -> GetServerBansRequest<'a> {
        GetServerBansRequest::new(self.0.clone(), server)
    }
    pub fn create_thread<'a>(
        &self,
        channel: &'a ChannelId,
        title: &'a str,
        content: &'a str,
    ) -> CreateThreadRequest<'a> {
        CreateThreadRequest::new(self.0.clone(), channel, title, content)
    }
    pub fn create_list_item<'a>(
        &self,
        channel: &'a ChannelId,
        message: &'a str,
    ) -> CreateListItemRequest<'a> {
        CreateListItemRequest::new(self.0.clone(), channel, message)
    }
    pub fn get_list_items<'a>(&self, channel: &'a ChannelId) -> GetListItemsRequest<'a> {
        GetListItemsRequest::new(self.0.clone(), channel)
    }
    pub fn get_list_item<'a>(
        &self,
        channel: &'a ChannelId,
        item: &'a ListId,
    ) -> GetListItemRequest<'a> {
        GetListItemRequest::new(self.0.clone(), channel, item)
    }
    pub fn update_list_item<'a>(
        &self,
        channel: &'a ChannelId,
        item: &'a ListId,
        message: &'a str,
    ) -> UpdateListItemRequest<'a> {
        UpdateListItemRequest::new(self.0.clone(), channel, item, message)
    }
    pub fn delete_list_item<'a>(
        &self,
        channel: &'a ChannelId,
        item: &'a ListId,
    ) -> DeleteListItemRequest<'a> {
        DeleteListItemRequest::new(self.0.clone(), channel, item)
    }
    pub fn complete_list_item<'a>(
        &self,
        channel: &'a ChannelId,
        item: &'a ListId,
    ) -> CompleteListItemRequest<'a> {
        CompleteListItemRequest::new(self.0.clone(), channel, item)
    }
    pub fn uncomplete_list_item<'a>(
        &self,
        channel: &'a ChannelId,
        item: &'a ListId,
    ) -> UncompleteListItemRequest<'a> {
        UncompleteListItemRequest::new(self.0.clone(), channel, item)
    }
    pub fn create_doc<'a>(
        &self,
        channel: &'a ChannelId,
        title: &'a str,
        content: &'a str,
    ) -> CreateDocRequest<'a> {
        CreateDocRequest::new(self.0.clone(), channel, title, content)
    }
    pub fn get_docs<'a>(&self, channel: &'a ChannelId) -> GetDocsRequest<'a> {
        GetDocsRequest::new(self.0.clone(), channel)
    }
    pub fn get_doc<'a>(&self, channel: &'a ChannelId, doc: &'a DocId) -> GetDocRequest<'a> {
        GetDocRequest::new(self.0.clone(), channel, doc)
    }
    pub fn update_doc<'a>(
        &self,
        channel: &'a ChannelId,
        doc: &'a DocId,
        title: &'a str,
        content: &'a str,
    ) -> UpdateDocRequest<'a> {
        UpdateDocRequest::new(self.0.clone(), channel, doc, title, content)
    }
    pub fn delete_doc<'a>(&self, channel: &'a ChannelId, doc: &'a DocId) -> DeleteDocRequest<'a> {
        DeleteDocRequest::new(self.0.clone(), channel, doc)
    }
    pub fn add_reaction<'a, C: Into<ContentId<'a>>>(
        &self,
        channel: &'a ChannelId,
        content: C,
        emote: &'a EmoteId,
    ) -> AddReactionRequest<'a> {
        AddReactionRequest::new(self.0.clone(), channel, content, emote)
    }
    pub fn award_member<'a>(
        &self,
        server: &'a ServerId,
        user: &'a UserId,
        amount: i32,
    ) -> MemberXpRequest<'a> {
        MemberXpRequest::new(self.0.clone(), server, user, amount)
    }
    pub fn award_role<'a>(
        &self,
        server: &'a ServerId,
        role: &'a RoleId,
        amount: i32,
    ) -> RoleXpRequest<'a> {
        RoleXpRequest::new(self.0.clone(), server, role, amount)
    }
    pub fn add_group_member<'a>(
        &self,
        group: &'a GroupId,
        user: &'a UserId,
    ) -> AddGroupMemberRequest<'a> {
        AddGroupMemberRequest::new(self.0.clone(), group, user)
    }
    pub fn delete_group_member<'a>(
        &self,
        group: &'a GroupId,
        user: &'a UserId,
    ) -> DeleteGroupMemberRequest<'a> {
        DeleteGroupMemberRequest::new(self.0.clone(), group, user)
    }
    pub fn get_member_roles<'a>(
        &self,
        server: &'a ServerId,
        user: &'a UserId,
    ) -> GetMemberRolesRequest<'a> {
        GetMemberRolesRequest::new(self.0.clone(), server, user)
    }
}
impl Deref for GuildedClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
