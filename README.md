# SpamWatchAPI

API server for SpamWatch

## Running via Docker

Before running `docker-compose` please make sure you have added:
* Postgres password in `/secrets/.postgres-passwd`
* Created `config.toml` from `example.config.toml`.

```
docker-compose up [-d]
```

