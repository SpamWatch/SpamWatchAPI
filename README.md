# SpamWatch API
This repository is where the source code lies for the SpamWatch API, written in Rust.

**NOTE: The SpamWatch API is in beta phrase, but it successfully worked in production.**

## Setup
### What You Need?
- Rust
- Postgres for database
- Web server with public-reachable address like `https://api.spamwat.ch`.

### Manual Builds
1. Get your copy of this code with `git clone https://github.com/SPamWatch/SpamWatchAPI.git`.
2. Install dependencies with `cargo build`.
3. Create your own configuration file by copying the `example.config.toml` file.
4. Open console type `cargo run` and hit the road!

### With Docker
Soon!

#### Using Docker Compose
Coming soon, refer to [this GitHub PR](https://github.com/SpamWatch/SpamWatchAPI/pull/2) for details.

## Need help?
See our docs in the 'docs' directory or visit `https://spamwatch-api-docs.devhubcentral.ml` (coming soon).

If you need assistance, create a issue in [GitHub](https://github.com/SpamWatch/SpamWatchAPI/issues/new) or in [GitLab](https://gitlab.com/MadeByThePinsTeam-DevLabs/forks/SpamWatchAPI/issues/new).

## License & Legal
See `LICENSE` file for details. For compliance framework in GitLab, we use GDPR as our regulatory standard.

## Who made this?
@SitiSchu

## TO-DO
* [ ] Documentation for the SpamWatch API server
* [ ] Dockerfiles for using SpamWatch API server in Docker
* [ ] Issue/PR templates
