export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${DB_HOST}:${DB_PORT}/${POSTGRES_DB}"
echo "DATABASE_URL: ${DATABASE_URL}"

cd metrix

cargo test
