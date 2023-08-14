# Trello → Discord webhook service
[![Build Status](https://github.com/riichi/trello-to-discord-webhook-service/workflows/Rust%20CI/badge.svg)](https://github.com/riichi/trello-to-discord-webhook-service/actions)
[![Docker Builds](https://github.com/riichi/trello-to-discord-webhook-service/workflows/Docker/badge.svg)](https://github.com/riichi/trello-to-discord-webhook-service/actions)

## Setting up
### Preliminaries

1. Set up a Trello Power-up at https://trello.com/power-ups/admin; keep the API key and secret, the rest is not
   important
2. Set up a Discord webhook at a channel of choice, keep the URL

### Config file

First, copy an example config file to `config.toml`:
```shell
cp config.example.toml config.toml
```

Then populate `api.api_key`, `api.api_secret`, and `discord.url` entries with values from the above section.
Set `trello.callback_url` to your webhook's URL.
`api.api_token` should remain unset for now.

### Getting API token

Power-up needs to be authorized to acces user's boards.
A helper subcommand for this process is available:
```shell
cargo run authorize --name "Your webhook power-up" --expiration never
```
A browser window should pop up - after authorizing the app you'll be redirected to a page with an API token.
Copy the token to the console window and press <kbd>Enter</kbd>.
The application should automatically save the token in `config.toml`.
To verify that the configuration is correct, call e.g.
```shell
cargo run get-boards
```
– a list of available board should be printed.

### Deploy the webhook

We have to do this before actually registering the webhook since Trello does some HTTP requests upon webhook creation to
verify that the service is working.
A self-contained Dockerfile (which keeps a copy of your `config.toml` inside the image) is provided for convenience.

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
