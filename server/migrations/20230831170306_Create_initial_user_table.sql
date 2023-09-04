-- Add migration script here
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    user_type VARCHAR(50),
    api_key VARCHAR(255)
)