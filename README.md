# Spam Watch API

API server for Spam Watch Bot

## Running via Docker

Before running `docker-compose` please make sure you have added:
* Postgres password in `/secrets/.postgre-passwd`
* Created `config.toml` from `example.config.toml`.

```
docker-compose up [-d]
```

