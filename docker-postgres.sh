#!/bin/bash

docker run \
  --name spam-watch-postgres \
  -p 5432:5432 \
  -e POSTGRES_PASSWORD=sub123 \
  -e POSTGRES_USER="spam-watch" \
  -e POSTGRES_DB="spam-watch-db" \
  postgres