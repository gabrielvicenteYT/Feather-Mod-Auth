-- Add migration script here

-- Tokens are a long-lived character string that is kept in the cookies for session conservation.
CREATE TABLE minos.tokens
(
    token   text PRIMARY KEY,
    expires timestamp DEFAULT CURRENT_TIMESTAMP + INTERVAL '3 months',
    user_id uuid,
    CONSTRAINT userid
        FOREIGN KEY (user_id)
            REFERENCES minos.users (id)

)