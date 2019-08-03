# This is dev version to make the Cargo builds fast and should re-build and run the whole
# app everytime the container is restarted, as Rust doesn't has a proper way of hot reloading
FROM rust:latest

WORKDIR /app/spam-watch-api

VOLUME ./ /app/spam-watch-api

# This will cache the library builds, which will ultimately help in
# re-running our apps faster.
RUN cargo build

CMD ["cargo", "run"]

