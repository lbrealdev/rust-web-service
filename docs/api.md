# API Reference

Base URL: `http://127.0.0.1:3030`

## Authentication

The frontend uses a simple password-based login via `POST /login`. Once logged in, the browser stores a flag in `localStorage` that gates create/delete actions. The API itself does not require authentication headers — the gating is purely client-side.

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

**Response** `200 OK`

```
Question added
```

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

| Field     | Type             | Required | Description          |
|-----------|------------------|----------|----------------------|
| `id`      | i32              | Yes      | Question ID          |
| `title`   | string           | Yes      | Updated title        |
| `content` | string           | Yes      | Updated content      |
| `tags`    | string[] \| null | No       | Updated tags         |

```json
{
  "id": 1,
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

---

### Delete Question

```
DELETE /questions/:id
```

**Path Parameters**

| Param | Type | Description      |
|-------|------|------------------|
| `id`  | i32  | Question ID      |

**Response** `200 OK`

```
Question 1 deleted
```

---

### Add Answer

```
POST /answers
Content-Type: application/x-www-form-urlencoded
```

> **Note**: This endpoint accepts form-encoded data, not JSON.

**Form Fields**

| Field         | Type   | Required | Description          |
|---------------|--------|----------|----------------------|
| `content`     | string | Yes      | Answer text          |
| `question_id` | i32    | Yes      | Parent question ID   |

```sh
curl -X POST 'http://127.0.0.1:3030/answers' \
  -d 'content=Only run things!!!' \
  -d 'question_id=1'
```

**Response** `200 OK`

```
Awnser added
```

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
