-- Your SQL goes here
CREATE TABLE CHATS (
    ID UUID PRIMARY KEY,
    NAME VARCHAR(255) NOT NULL,
    CREATED_AT TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UPDATED_AT TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)