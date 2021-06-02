
use crate::models::DbDateTime;



use serde::{Deserialize, Serialize};
use sqlx::{FromRow};

use uuid::Uuid;

/// This struct represnts the database version of the oauth client.
/// The table linked to this model is `minos.oauth_clients`
#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct OAuthClient {
    /// Ths id of the OAuth client.
    /// This is both used as the primary key of the database and as the client_id in the oauth protocol.
    pub id: String,
    /// The secret of the OAuth client.
    /// This is a hashed representation of the secret, using the One-Way hashing system available on [crate::utils::password_hashing]
    #[sqlx(default)]
    pub secret: Option<String>,
    /// The owner of this client.
    /// This is a foreign key for the `minos.users (id)` row.
    pub owner: Uuid,
    /// The date of creation of the client
    /// Defaults to the current date and time.
    pub creation_date: DbDateTime,
    /// Whether the client is enabled for public usage
    /// If this field is set to false, only the [OAuthClient::owner] can proceed the process without getting an error
    pub enabled: bool,
    /// The display name of the client.
    /// This will be displayed on many places during the authentication process
    pub name: String,
    /// The description.
    /// This has to be small, and explain how the connection is gonna be used.
    pub description: String,
    /// A link to the Client's privacy policy page.
    /// This is gonna be used in the oauth consent step, to allow for easy access for users.
    pub privacy_policy: Option<String>,
    /// A link to the Client's terms of service page.
    /// This is gonna be used in the oauth consent step, to allow for easy access for users.
    pub terms_of_service: Option<String>,
    /// A link to the icon for the Client.
    /// This will be displayed in the oauth consent step, as an easy way to recognise the authorizing application.
    pub icon: Option<String>,
    /// JS domains
    /// This field is the list of allowed origins for a client-side JS request.
    /// If an authentication request is made and the domain is not in this list, the request will be denied.
    pub js_domains: Vec<String>,
    /// List of authorized callback domains.
    /// If the callback url is not present in this list while making an authentication request, it will be denied.
    pub callback_urls: Vec<String>,
    /// Whether the app is a first party.
    /// A first party app is an app that is controlled by the party hosting the instance.
    /// It allows to bypass the consent step for OAuth, among other things.
    pub first_party: bool,
    /// A list of special scope granted to the app.
    /// Special scopes are scopes that are voluntarly restricted to approved clients, as they may:
    /// - Expose private informations
    /// - Do actions that a non authorized app should not make (like deleting an account)
    /// - Access money (any kind)
    /// Among other things.
    pub special_scopes: Vec<String>,
    /// A reason for the disabling of the client.
    /// This field can be None in in the event of a user desactivation, but it has to be filled if the action comes from the administrators.
    /// This is displayed in the error message displayed to the client.
    pub disabled_reason: Option<String>,
}

impl OAuthClient {}
