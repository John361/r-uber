#!/bin/bash

# Variables
RUBER_HOME_PATH="/opt/r-uber"
RUBER_LOG_PATH="/var/log/r-uber"

# Create storage directories
sudo mkdir ${RUBER_LOG_PATH}

sudo mkdir -p /data/r-uber/kafka
sudo chown -R 1001:1001 /data/r-uber/kafka


# Create service
sudo useradd --system --create-home --home-dir ${RUBER_HOME_PATH} --comment "R-uber system user" --shell /sbin/nologin r-uber

cat << EOF > ${RUBER_HOME_PATH}/r-uber.service
[Unit]
Description=R-uber service
Documentation=https://github.com/John361/r-uber

[Service]
Type=simple
User=r-uber
Group=r-uber
RootDirectory=/
WorkingDirectory=${RUBER_HOME_PATH}
Environment=RUST_LOG=Info
ExecStart=${RUBER_HOME_PATH}/r-uber start -c ${RUBER_HOME_PATH}/config.json
StandardOutput=append:${RUBER_LOG_PATH}/r-uber.log
StandardError=append:${RUBER_LOG_PATH}/r-uber.log
RestartSec=3
Restart=no

[Install]
WantedBy=multi-user.target
EOF

sudo mv ${RUBER_HOME_PATH}/r-uber.service /etc/systemd/system/
sudo touch ${RUBER_HOME_PATH}/config.json


# Configure log rotation
cat << EOF > ${RUBER_HOME_PATH}/r-uber-log
${RUBER_LOG_PATH}/r-uber.log {
    daily
    missingok
    rotate 7
    dateext
    dateformat -%Y%m%d
    compress
    delaycompress
    notifempty
    ifempty
    create 0640 r-uber r-uber
    postrotate
        systemctl restart r-uber.service
    endscript
}
EOF

sudo mv ${RUBER_HOME_PATH}/r-uber-log ${RUBER_LOG_PATH}/r-uber


# Start service
sudo systemctl daemon-reload
sudo systemctl start r-uber
sudo systemctl status r-uber
