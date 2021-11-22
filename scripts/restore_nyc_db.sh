#!/usr/bin/env bash

set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: $(psql) is not installed."
    exit 1
fi

if [ ! -d data ]; then
    mkdir data
    # Download sample data used in the PostGIS workshop
    curl http://s3.cleverelephant.ca/postgis-workshop-2020.zip -o workshop.zip
    # Extract only the backup to the `data` directory
    unzip -j workshop.zip "postgis-workshop/data/nyc_data.backup" -d data
    rm workshop.zip
fi

# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER="${POSTGRES_USER:=postgres}"
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# Check if a custom password has been set, otherwise default to 'nyc'
DB_NAME="${POSTGRES_DB:=nyc}"
# Check if a custom port has been set, otherwise default to '5432'
DB_PORT="${POSTGRES_PORT:=5432}"
# Check if a custom host has been set, otherwise default to 'localhost'
DB_HOST="${POSTGRES_HOST:=localhost}"

# Create the database
PGPASSWORD="${DB_PASSWORD}" psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" \
    -c "CREATE DATABASE ${DB_NAME};"

# Load the PostGIS spatial extension and print the version
PGPASSWORD="${DB_PASSWORD}" psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "${DB_NAME}" \
    -c "CREATE EXTENSION postgis; SELECT postgis_full_version();"

# Restore the data
PGPASSWORD="${DB_PASSWORD}" pg_restore -d "${DB_NAME}" -U "${DB_USER}" --no-owner data/nyc_data.backup

echo "Database ${DB_NAME} restored!"
