// This file is auto generated by `cg` <https://github.com/teloxide/cg> (24572cd + local changes).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::ChatId;

impl_payload! {
    /// Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns the revoked invite link as [`ChatInviteLink`] object.
    ///
    /// [`ChatInviteLink`]: crate::types::ChatInviteLink
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub RevokeChatInviteLink (RevokeChatInviteLinkSetters) => String {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: ChatId [into],
            /// The invite link to revoke
            pub invite_link: String [into],
        }
    }
}
