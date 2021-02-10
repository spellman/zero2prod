#!/usr/bin/env bash
set -x
set -eo pipefail

# Check if a customer user has been set. Otherwise, default to "postgres".
DB_USER="${POSTGRES_USER:=postgres}"

# Check if a custom password has been set. Otherwise, default to "password".
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"

# Check if a custom database name has been set. Otherwise, default to "newsletter".
DB_NAME="${POSTGRES_DB:=newsletter}"

# Check if a custom port has been set. Otherwise, default to "5432".
DB_PORT="${POSTGRES_PORT:=5432}"

# Launch postgres using Docker.
if [[ -z "${SKIP_DOCKER}" ]]
then
  docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000 # Increased maximum number of connnections for testing purposes.
fi

# Ping postgres until it's ready to accept commands.
# Without this, the following `sqlx database create` may be run before postgres is ready.
export PGPASSWORD="${DB_PASSWORD}"

until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is not yet available - sleeping for 1 second before retrying..."
  sleep 1
done

>&2 echo "Postgres is up and running and available (as per successful ping) on port ${DB_PORT}."

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated. We're ready to go."