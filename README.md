# Rust web service

## Use

Run the web-server:
```shell
cargo run
```

Access your rust web-server via browser:
```text
http://localhost:3030

http://localhost:3030/questions
```


### Get questions
```shell
curl "http://localhost:3030/questions?start=1&end=200"

```

### Create a new question
```shell
curl -L \
    -X POST \
    'http://localhost:3030/questions' \
    -H 'Content-type: application/json' \
    -d '{"id": "2", "title": "New question", "content": "How does this work again?"}'
```

### Update a question
```shell
curl -L \
  -X PUT \
  'http://localhost:3030/questions/2' \
  -H 'Content-type: application/json' \
  -d '{
        "id": "2",
        "title": "White Collar Criminal",
        "content": "Midnite"
    }'
```


