# This is dev version to make the Cargo builds fast and should re-build and run the whole
# app everytime the container is restarted, as Rust doesn't has a proper way of hot reloading

# Keeping Rust to it's latest version for having the latest Rust Dependency.
# Once async/await is introduced, we can hard-code the version.
FROM rust:latest

WORKDIR /app/spamwatch-api

COPY . /app/spamwatch-api

# This will cache the library builds, which will ultimately help in
# re-running our apps faster.
RUN cargo build

ENV RUST_BACKTRACE 1

CMD ["cargo", "run"]

