# Rust web service

A rust web service using warp and tokio.

## Usage

Run the web-server:
```shell
just run
```

Once the server is running, you can access through the following URLs:

| **Endpoints**                   |
|---------------------------------|
|      http://localhost:3030      |
| http://localhost:3030/questions |

Get questions:
```shell
curl -sL \
  -H 'Content-type: application/json' \
  'http://localhost:3030/questions' | jq .
```

Get questions with query:
```shell
curl -sL 'http://localhost:3030/questions?offset=1&limit=200' | jq .
```

Create a new question:
```shell
curl -w '\n' -L \
 -X POST \
 -H 'Content-type: application/json' \
 'http://localhost:3030/questions' \
 -d '{
      "id": "2",
      "title": "New question",
      "content": "How does this work again?"
    }'
```

Create a new question (updated):
```shell
curl -v -L -w '\n' \
  -H 'Content-type: application/json' \
  'http://localhost:3030/questions' \
  -d '{
        "title": "test - first question",
        "content": "How does this work again?"
      }'
```

Add answer:
```shell
curl -v -L -w '\n' \
  -H 'Content-type: application/json' \
  'http://localhost:3030/answers' \
  -d '{
        "id": "2",
        "content": "Only run things!!"
        "question_id": "1"
     }'
```

Update a question:
```shell
curl -L -w '\n' \
  -X PUT \
  -H 'Content-type: application/json' \
  'http://localhost:3030/questions/2' \
  -d '{
        "id": 2,
        "title": "White Collar Criminal",
        "content": "Akae Beka"
      }'
```

Delete a question:
```shell
curl -L -w '\n' \
  -X DELETE \
  -H 'Content-type: application/json' \
  'http://localhost:3030/questions/1'
```

Tree project excluding target/ directory:
```shell
tree -I target
```

## Setup local database

Pull the `postgres` docker image:
```shell
docker pull postgres
```

Create an `.env` file with psql password:
```shell
echo 'POSTGRES_PASSWORD="localpsql2025"' > .env
```


```shell

```

## Chapter 5

Create a new library in Rust:
```shell
cargo new handle-errors --lib
```

## Chapter 6

```shell
RUST_LOG=info cargo run

RUST_LOG=debug cargo run

RUST_LOG=info cargo run 2>logs.txt
```

```shell
curl -L -X GET 'localhost:3030/questions'
```

## Chapter 7
