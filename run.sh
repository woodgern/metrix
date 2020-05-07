export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${DB_HOST}:${DB_PORT}/${POSTGRES_DB}"

# Required for Database pooling configuration
# https://api.rocket.rs/v0.4/rocket_contrib/databases/index.html
export ROCKET_DATABASES="{postgres_db={url=\"${DATABASE_URL}\",pool_size=10}}"

echo "DATABASE_URL: ${DATABASE_URL}"
echo "ROCKET_DATABASES: ${ROCKET_DATABASES}"

cargo run
