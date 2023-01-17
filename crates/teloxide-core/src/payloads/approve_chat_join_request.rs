//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{Recipient, True, UserId};

impl_payload! {
    /// Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the _can_invite_users_ administrator right. Returns _True_ on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub ApproveChatJoinRequest (ApproveChatJoinRequestSetters) => True {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: Recipient [into],
            /// Unique identifier of the target user
            pub user_id: UserId,
        }
    }
}