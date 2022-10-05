#!/bin/bash

export POSTGRES_USER=postgres
export POSTGRES_PASSWORD="$3cr3+"
export POSTGRES_DB_URL=0.0.0.0
export POSTGRES_DB=error_microservice
export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_DB_URL}/${POSTGRES_DB}"

export ENV=LOCAL
export HOST=0.0.0.0
export PORT=8080
export LOG_LEVEL=trace
