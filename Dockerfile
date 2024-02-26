FROM rust:1.76-slim-bookworm as builder

COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim

RUN apt update -y && apt install -y ca-certificates && apt-get clean && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/trello-webhook /usr/local/bin/trello-webhook
RUN mkdir /app
WORKDIR "/app"

EXPOSE 3000
CMD ["trello-webhook", "start-webhook"]
