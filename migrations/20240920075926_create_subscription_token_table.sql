-- Add migration script here
CREATE TABLE subscriptions_tokens
(
    subscription_token TEXT PRIMARY KEY,
    subscription_id    uuid NOT NULL REFERENCES subscriptions (id)
)