# !/usr/bin/env bash
# set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed"
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed"
  echo >&2 "Use: "
  echo >&2 "     cargo install --version='~0.8' sqlx-cli \
  --no-default-features --features rustls,postgres"
  echo >&2 "to install it"
  exit 1
fi

# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER="${POSTGRES_USER:=postgres}"
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# Check if a custom database name has been set, otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"
# Check if a custom port has been set, otherwise default to '5432'DB_PORT=
DB_PORT="${POSTGRES_PORT:=5432}"
# Check if a custom host has been set, otherwise default to 'localhost'
DB_HOST="${POSTGRES_HOST:=localhost}"

function start_container() {
  docker start ${CONTAINER_NAME}
}

function run_container() {
  docker run \
    --name ${CONTAINER_NAME} \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000
    # ^ Increased maximum number of connections for testing purposes
}

CONTAINER_NAME="zero2prod_db"
# Check if the container exists
if [ "$(docker ps -a -q -f name=${CONTAINER_NAME})" ]; then
  # Check if the container is running
  if [ "$(docker ps -q -f name=${CONTAINER_NAME})" ]; then
    >&2 echo "Container ${CONTAINER_NAME} is already running. skipping"
  else
    >&2 echo "Container ${CONTAINER_NAME} exists but is not running."
    start_container
  fi
else
  >&2 echo "Creating Postgres container..."
  run_container
fi

export PGPASSWORD="${DB_PASSWORD}"
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}"

DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
export DATABASE_URL

sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated. Ready to roll!"