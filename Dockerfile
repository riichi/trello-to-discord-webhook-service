FROM rust:1.76-bookworm as builder

RUN mkdir /app
WORKDIR "/app"
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim

# hadolint ignore=DL3008
RUN apt-get update -y && \
    apt-get install --no-install-recommends -y ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/trello-webhook /usr/local/bin/trello-webhook
RUN mkdir /app
WORKDIR "/app"

EXPOSE 3000
CMD ["trello-webhook", "start-webhook"]
