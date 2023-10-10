# R-uber
[![R-uber build](https://github.com/John361/r-uber/actions/workflows/build.yml/badge.svg)](https://github.com/John361/r-uber/actions/workflows/build.yml)
[![R-uber release](https://github.com/John361/r-uber/actions/workflows/release.yml/badge.svg)](https://github.com/John361/r-uber/actions/workflows/release.yml)

Tool written in Rust that can listening in multiple folders recursively for files and move them in other folders

# Development
## Docker
```shell
bash tools/init.sh
docker-compose -f tools/docker/docker-compose.yml up --build
```

## Config file
```json
{
    "appName": "R-uber",
    "racesPath": "full/path/to/races.json",
    "kafka": {
        "hosts": ["localhost:9094"],
        "topic": "uber-race"
    }
}
```

## Races file
See schema in tools/schemas/races-schema.json

## Rust
```
RUST_LOG=Info cargo run --package r-uber --bin r-uber start -c full/path/to/r-uber/config.json
```
