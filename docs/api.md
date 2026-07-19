# API Reference

Base URL: `http://127.0.0.1:3030`

## Authentication

The frontend uses a simple password-based login via `POST /login`. Once logged in, the browser stores a flag in `localStorage` that gates create/delete actions. The API itself does not require authentication headers — the gating is purely client-side.

`ADMIN_PASSWORD` must be set in the environment (loaded from `.env`).

---

## Endpoints

### List Questions

```
GET /questions
GET /questions?limit=10&offset=0
```

**Query Parameters**

| Param    | Type | Default | Description          |
|----------|------|---------|----------------------|
| `limit`  | i32  | —       | Max items (requires `offset`) |
| `offset` | i32  | 0       | Starting index       |

**Response** `200 OK`

```json
[
  {
    "id": 1,
    "title": "How?",
    "content": "Please help!",
    "tags": ["general", "help"]
  }
]
```

---

### Get Question

```
GET /questions/:id
```

**Path Parameters**

| Param | Type | Description |
|-------|------|-------------|
| `id`  | i32  | Question ID |

**Response** `200 OK`

```json
{
  "id": 1,
  "title": "How?",
  "content": "Please help!",
  "tags": ["general"]
}
```

**Response** `404 Not Found` — question does not exist

---

### List Answers for Question

```
GET /questions/:id/answers
```

**Path Parameters**

| Param | Type | Description |
|-------|------|-------------|
| `id`  | i32  | Question ID |

**Response** `200 OK`

```json
[
  {
    "id": 1,
    "content": "Try this…",
    "question_id": 1
  }
]
```

---

### Create Question

```
POST /questions
Content-Type: application/json
```

**Request Body**

| Field     | Type             | Required | Description          |
|-----------|------------------|----------|----------------------|
| `title`   | string           | Yes      | Question title       |
| `content` | string           | Yes      | Question body        |
| `tags`    | string[] \| null | No       | Tag list             |

```json
{
  "title": "New question",
  "content": "How does this work?",
  "tags": ["rust", "warp"]
}
```

**Response** `201 Created`

```json
{
  "id": 1,
  "title": "New question",
  "content": "How does this work?",
  "tags": ["rust", "warp"]
}
```

**Response** `400 Bad Request` — empty `title` or `content`

---

### Update Question

```
PUT /questions/:id
Content-Type: application/json
```

**Path Parameters**

| Param | Type | Description      |
|-------|------|------------------|
| `id`  | i32  | Question ID      |

**Request Body**

The question id comes from the path only (not the body).

| Field     | Type             | Required | Description          |
|-----------|------------------|----------|----------------------|
| `title`   | string           | Yes      | Updated title        |
| `content` | string           | Yes      | Updated content      |
| `tags`    | string[] \| null | No       | Updated tags         |

```json
{
  "title": "Updated title",
  "content": "Updated content",
  "tags": ["updated"]
}
```

**Response** `200 OK`

```json
{
  "id": 1,
  "title": "Updated title",
  "content": "Updated content",
  "tags": ["updated"]
}
```

**Response** `400 Bad Request` — empty `title` or `content`

**Response** `404 Not Found` — question does not exist

---

### Delete Question

```
DELETE /questions/:id
```

**Path Parameters**

| Param | Type | Description      |
|-------|------|------------------|
| `id`  | i32  | Question ID      |

Deletes the question and any related answers (`ON DELETE CASCADE`).

**Response** `200 OK`

```
Question 1 deleted
```

**Response** `404 Not Found` — question does not exist

---

### Add Answer

```
POST /answers
Content-Type: application/json
```

**Request Body**

| Field         | Type   | Required | Description          |
|---------------|--------|----------|----------------------|
| `content`     | string | Yes      | Answer text          |
| `question_id` | i32    | Yes      | Parent question ID   |

```json
{
  "content": "Only run things!!!",
  "question_id": 1
}
```

**Response** `201 Created`

```json
{
  "id": 1,
  "content": "Only run things!!!",
  "question_id": 1
}
```

**Response** `400 Bad Request` — empty `content`

---

### Delete Answer

```
DELETE /answers/:id
```

**Path Parameters**

| Param | Type | Description |
|-------|------|-------------|
| `id`  | i32  | Answer ID   |

**Response** `200 OK`

```
Answer 1 deleted
```

**Response** `404 Not Found` — answer does not exist

---

### Login

```
POST /login
Content-Type: application/json
```

**Request Body**

| Field      | Type   | Required | Description         |
|------------|--------|----------|---------------------|
| `password` | string | Yes      | Admin password      |

```json
{
  "password": "your-password"
}
```

**Response** `200 OK` (success)

```json
{
  "success": true,
  "message": "Login successful"
}
```

**Response** `401 Unauthorized` (failure)

```json
{
  "success": false,
  "message": "Invalid password"
}
```
