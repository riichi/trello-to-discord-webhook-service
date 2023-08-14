FROM rust:1.71 as builder

COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim

RUN apt update -y && apt install -y ca-certificates && apt-get clean && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/trello-webhook /usr/local/bin/trello-webhook
WORKDIR "/app"

EXPOSE 3000
CMD ["trello-webhook", "start-webhook"]
