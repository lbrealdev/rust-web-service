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



```shell
curl "http://localhost:3030/questions?start=1&end=200"

```


```shell
curl -L \
    -X POST \
    'http://localhost:3030/questions' \
    -H 'Content-type: application/json' \
    -d '{"title": "New question", "content": "How does this work again?"}'
```

```shell
curl -L \
  -X PUT \
  'http://localhost:3030/questions' \
  -H 'Content-type: application/json' \
  -d '{"id": "1", "title": "New question", "content": "How does this work again?"}'
```

