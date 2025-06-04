# Rust web service

A rust web service using warp and tokio.

## Usage

Run the web-server:
```shell
cargo run
```

Once the server is running, you can access through the following URLs:

| **Endpoints**                   |
|---------------------------------|
|      http://localhost:3030      |
| http://localhost:3030/questions |

### Get questions

Using jq:
```shell
curl -sL \ 
  -H 'Content-type: application/json' \ 
  'http://localhost:3030/questions' | jq .
```

```shell
curl -sL 'http://localhost:3030/questions?offset=1&limit=200' | jq .
```

### Create a new question
```shell
curl -L \ 
  -X POST \ 
  -H 'Content-type: application/json' \ 
  'http://localhost:3030/questions'
  -d '{
        "id": "2",
        "title": "New question",
        "content": "How does this work again?"
      }'
```

### Create a new question - Updated
```shell
curl -v -L \ 
  -H 'Content-type: application/json' \ 
  'http://localhost:3030/questions' \ 
  -d '{
        "title": "test - first question",
        "content": "How does this work again?"
      }'
```

### Update a question
```shell
curl -L \ 
  -X PUT \ 
  -H 'Content-type: application/json' \ 
  'http://localhost:3030/questions/2' \ 
  -d '{
        "id": 2,
        "title": "White Collar Criminal",
        "content": "Midnite"
      }'
```

### Delete a question
```shell
curl -L \ 
  -X DELETE \ 
  -H 'Content-type: application/json' \ 
  'http://localhost:3030/questions/1'
```

Tree project excluding target/ directory:
```shell
tree -I target
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
