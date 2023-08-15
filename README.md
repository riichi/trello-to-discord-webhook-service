# Trello → Discord webhook service
[![Build Status](https://github.com/riichi/trello-to-discord-webhook-service/workflows/Rust%20CI/badge.svg)](https://github.com/riichi/trello-to-discord-webhook-service/actions)
[![Docker Builds](https://github.com/riichi/trello-to-discord-webhook-service/workflows/Docker/badge.svg)](https://github.com/riichi/trello-to-discord-webhook-service/actions)

## Setting up
### Preliminaries

1. Set up a Trello Power-up at https://trello.com/power-ups/admin; keep the API key and secret, also authorize the
   power-up to get the API token (click `Token` next to the `API key` field in power-up settings)
2. Set up a Discord webhook at a channel of choice, keep the URL

### Configuration

Configuration is done with environment variables (cf. `config.rs`, `.env.example`):

* `API_KEY`, `API_SECRET`, `API_TOKEN`: Trello API credentials
* `DISCORD_URL`: Discord webhook URL
* `WEBHOOK_PORT`: which port to listen to (default 3000)
* `WEBHOOK_URL`: Internet-accessible endpoint URL. Used to register the webhook at Trello and to verify HMAC signatures
  of webhook event requests.

To verify that the variables are set properly, call e.g.
```shell
cargo run get-boards
```
– a list of available boards should be printed.

If you are using the provided Compose file, you can simply copy `.env.example` to `.env` and fill in all missing values.

### Deploy the webhook

We have to do this before actually registering the webhook since Trello does some HTTP requests upon webhook creation to
verify that the service is working.
The service has to be running and be available under `WEBHOOK_URL`.

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
