-- Oauth Clients are the basis for the oauth2 flow.
CREATE TABLE minos.oauth_clients
(
    id               text PRIMARY KEY NOT NULL,
    -- The secret is hashed to not be exposed
    secret           BYTEA            NOT NULL,

    owner            UUID             NOT NULL,
    creation_date    timestamp        NOT NULL DEFAULT CURRENT_TIMESTAMP,
    enabled          BOOLEAN          NOT NULL DEFAULT false,
    name             varchar(64)      NOT NULL,
    description      varchar(1024)    NOT NULL DEFAULT '',
    privacy_policy   varchar(1024),
    terms_of_service varchar(1024),
    icon             varchar(512),
    js_domains       varchar(512)[]   NOT NULL DEFAULT array[]::varchar[],
    callback_urls    varchar(1024)[]  NOT NULL DEFAULT array[]::varchar[],
    first_party      BOOLEAN          NOT NULL DEFAULT false,
    special_scopes   text[]           NOT NULL DEFAULT array[]::text[],


    CONSTRAINT userid
        FOREIGN KEY (owner)
            REFERENCES minos.users (id)
);

CREATE TABLE minos.authorization
(
    authorization_id   text      NOT NULL PRIMARY KEY,
    client_id          text      NOT NULL,
    user_id            UUID      NOT NULL,
    scopes             text[]    NOT NULL DEFAULT array[]::text[],
    authorization_date timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expired            bool      NOT NULL DEFAULT FALSE,
    CONSTRAINT clientid
        FOREIGN KEY (client_id)
            REFERENCES minos.oauth_clients (id),
    CONSTRAINT userid
        FOREIGN KEY (user_id)
            REFERENCES minos.users (id)
);
-- This is in another table to allow for one-to-many relationship
CREATE TABLE minos.refresh_tokens
(
    key              text NOT NULL PRIMARY KEY,
    authorization_id text NOT NULL,
    desactivated     bool NOT NULL DEFAULT false,

    CONSTRAINT authorizationid
        FOREIGN KEY (authorization_id)
            REFERENCES minos.authorization
);