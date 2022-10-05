#!/usr/bin/env bash
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 3650 -nodes
chmod 700 "${key_name}.key"
