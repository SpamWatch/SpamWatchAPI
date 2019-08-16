# SpamWatchAPI

API server for SpamWatch

## Running via Docker

Before running `docker-compose` please make sure you have added:
* Postgres password in `/secrets/.postgres-passwd`
* Create `config.toml` from `example.config.toml`.
  - Do change the hostname for [database] config to `host = "postgres"`

```
docker-compose up [-d]
```

