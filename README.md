# Usage


## Chat API Documentation

This API provides a set of endpoints to manage chats and conversations. 

## Models

### Chat

A chat is represented by the following fields:

- `id`: A unique identifier for the chat (UUID).
- `name`: The name of the chat.
- `created_at`: The date and time when the chat was created.
- `updated_at`: The date and time when the chat was last updated.

### Conversation

A conversation is represented by the following fields:

- `id`: A unique identifier for the conversation (UUID).
- `role`: The role of the user in the conversation. It can be either "Human" or "Assistant".
- `chat_id`: The identifier of the chat to which the conversation belongs.
- `message`: The content of the conversation.
- `created_at`: The date and time when the conversation was created.
- `updated_at`: The date and time when the conversation was last updated.

## API Endpoints

### List Chats

- **URL**: `/chats/`
- **Method**: `GET`
- **Response Content-Type**: `application/json`
- **Description**: Returns a list of all chats.
- **Response Example**:

```json
[
    {
        "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
        "name": "Chat 1",
        "created_at": "2023-08-05T14:48:00.000Z",
        "updated_at": "2023-08-05T14:48:00.000Z"
    },
    ...
]
```

### Get Chat

- **URL**: `/chats/<id>`
- **Method**: `GET`
- **Response Content-Type**: `application/json`
- **Description**: Returns a specific chat identified by `id`.
- **Response Example**:

```json
{
    "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "name": "Chat 1",
    "created_at": "2023-08-05T14:48:00.000Z",
    "updated_at": "2023-08-05T14:48:00.000Z"
}
```

### Create Chat

- **URL**: `/chats/create`
- **Method**: `POST`
- **Response Content-Type**: `application/json`
- **Request Body**:

```json
{
    "name": "string"
}
```

- **Description**: Creates a new chat with the provided name.

### Delete Chat

- **URL**: `/chats/delete/<id>`
- **Method**: `DELETE`
- **Response Content-Type**: `application/json`
- **Description**: Deletes the chat identified by `id`.

### Update Chat

- **URL**: `/chats/update/<id>`
- **Method**: `PUT`
- **Response Content-Type**: `application/json`
- **Request Body**:

```json
{
    "name": "string",
    "updated_at": "datetime"
}
```

- **Description**: Updates the chat identified by `id` with the provided name and updated_at time.

### Create Conversation

- **URL**: `/conversations/create`
- **Method**: `POST`
- **Response Content-Type**: `application/json`
- **Request Body**:

```json
{
    "chat_id": "uuid",
    "role": "string",
    "message": "string"
}
```

- **Description**: Creates a new conversation in the chat identified by `chat_id`, with the provided role and message. The `role` should be either "Human" or "Assistant".

### Inference

- **URL**: `/conversations/inference/<id>`
- **Method**: `GET`
- **Description**: This is a Server-Sent Events (SSE) endpoint that provides real-time updates for the conversation identified by `id`.

### Inference Internet

- **URL**: `/conversations/inference_internet/<id>`
- **Method**: `GET`
- **Description**: This is a Server-Sent Events (SSE) endpoint that provides real-time updates for the conversation identified by `id`, including internet-based inference.
