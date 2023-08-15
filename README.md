# Trello → Discord webhook service
[![Build Status](https://github.com/riichi/trello-to-discord-webhook-service/workflows/Rust%20CI/badge.svg)](https://github.com/riichi/trello-to-discord-webhook-service/actions)
[![Docker Builds](https://github.com/riichi/trello-to-discord-webhook-service/workflows/Docker/badge.svg)](https://github.com/riichi/trello-to-discord-webhook-service/actions)

## Setting up
### Preliminaries

1. Set up a Trello Power-up at https://trello.com/power-ups/admin; keep the API key and secret, also authorize the
   power-up to get the API token (click `Token` next to the `API key` field in power-up settings)
2. Set up a Discord webhook at a channel of choice, keep the URL

### Configuration

First, copy an example config file to `config.toml`:
```shell
cp config.example.toml config.toml
```

Then populate `api.key`, `api.secret`, `api.token`, and `discord.url` entries with values from the above
section.
Set `webhook.url` to your webhook's URL.

To verify that the Trello configuration is correct, call e.g.
```shell
cargo run get-boards
```
– a list of available boards should be printed.

> :information_source: Settings can be also overridden with environment variables, e.g. exporting `WEBHOOK_PORT=8080`
> will cause the service to listen on port 8080 regardless of `config.toml`'s contents.

### Deploy the webhook

We have to do this before actually registering the webhook since Trello does some HTTP requests upon webhook creation to
verify that the service is working.
The service has to be running and be available under `webhook.url`.

### Create webhook

First, get the ID of the board from the output of
```shell
cargo run get-boards
```
.
Then create the webhook:
```shell
cargo run create-webhook --description "Your webhook" --board-id "<BOARD ID>"
```

The service should start reporting the events.
