-- Your SQL goes here
CREATE TABLE CONVERSATIONS (
    ID UUID PRIMARY KEY,
    ROLE VARCHAR(255) NOT NULL,
    CHAT_ID UUID NOT NULL,
    MESSAGE TEXT NOT NULL,
    CREATED_AT TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UPDATED_AT TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT FK_CONVERSATIONS_CHATS FOREIGN KEY (CHAT_ID) REFERENCES CHATS(ID)
)