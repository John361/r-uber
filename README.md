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

# Production
## Docker
Get usable compose file from tools/docker folder or use your own

## Config file
Populate created file in /opt/r-uber or use another by modifying /etc/systemd/system/r-uber.service file
Always use full paths

## Races file
See schema in tools/schemas/races-schema.json
Always use full paths

## Rust
Download release, unzip it
```
bash init.sh
systemctl status r-uber.service
tail -f /var/log/r-uber/r-uber.log
```
