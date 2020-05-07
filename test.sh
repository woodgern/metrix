export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${DB_HOST}:${DB_PORT}/${POSTGRES_DB}"
export ROCKET_DATABASES="{postgres_db={url=\"${DATABASE_URL}\",pool_size=10}}"

echo "DATABASE_URL: ${DATABASE_URL}"
echo "ROCKET_DATABASES: ${ROCKET_DATABASES}"

cargo test
