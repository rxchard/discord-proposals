# Build stage
FROM rust:slim-buster AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo install --path .

# Production stage
FROM debian:buster-slim

COPY --from=builder /usr/local/cargo/bin/discord-proposals /usr/local/bin/discord-proposals
CMD [ "discord-proposals" ]
