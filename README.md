# RCLI

## rcli text encrypt

```shell
cargo run -- text encrypt --key fixtures/chacha20_key.txt --input  fixtures/cc.in
# 3AdrmI4nfbgfw0vNHrruFKs1W-qrsYAPLZIEfSg6USoAI6xLC52m0oQ6sjvTbuc
```

## rcli text decrypt

```shell
cargo run -- text decrypt --key fixtures/chacha20_key.txt --input  fixtures/cc.out
# hello from chacha20
```

## rcli jwt sign

```shell
cargo run -- jwt sign --sub acme --aud device1 --exp 1m
# jwt.io
# eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhY21lIiwiYXVkIjoiZGV2aWNlMSIsImV4cCI6MTcxNzg5MjAwNH0.fa7iJpl4dqoGSWMBsiVnd7MwUGlc3Yu5CAOgVhi8OPI
```

## rcli jwt verify

```shell
cargo run -- jwt verify -t eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhY21lIiwiYXVkIjoiZGV2aWNlMSIsImV4cCI6MTcxNzg5MjAwNH0.fa7iJpl4dqoGSWMBsiVnd7MwUGlc3Yu5CAOgVhi8OPI
```

## directory index

```shell
cargo run -- http serve
# http://127.0.0.1:8080/src
```
