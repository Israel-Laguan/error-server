#!/bin/bash
createuser -s $POSTGRES_USER
psql -U $POSTGRES_USER -d $POSTGRES_DB -a -f /app/scripts/db/dump.sql
createdb -U postgres $POSTGRES_DB
