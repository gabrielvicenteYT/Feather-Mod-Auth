use crate::models::DbDateTime;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
/// An authorization is the link between an user and a client.
/// This is the model that retains an approval from a user to a client, using the allowed scopes.
#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Authorization {
    /// This is an unique ID for the authorization. This is not given to the client or the user.
    pub authorization_id: String,
    /// The client id of the authorization. Foreign key of `minos.oauth_clients`
    pub client_id: String,
    /// The user id of the authorization. Foreign key of `minos.users`
    pub userid: Uuid,
    /// The list of scopes an user has agreed to in this authorisation.
    /// This list is checked with the requested scopes, and in the event that they do not matches,
    /// the process will ask for the updated scopes.
    pub scopes: Vec<String>,
    /// The date of creation of that authorization.
    /// This stores the time where the authorization is created for the first time.
    pub authorization_date: DbDateTime,
    /// The date of the last login on that authorization.
    /// This is refreshed each time an auth request is made using that client & user.
    pub login_date: DbDateTime,
    /// Whether the Authorization is expired.
    /// This can happen in the event that a client secret is breached,
    /// or the user revokes all applications.
    pub expired: bool,
}
