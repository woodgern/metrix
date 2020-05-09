.DEFAULT_GOAL:=help
.PHONY: help server tests

help: ## Show all the available make commands
	@echo "\n======================================================================================================================================================================="
	@awk '/```ascii/{a=1; next}/```/{a=0}(a==1){print}' README.md
	@echo "=======================================================================================================================================================================\n"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

server: ## start docker/server
	test -f /.dockerenv || docker-compose build metrix
	test -f /.dockerenv || docker-compose run --service-ports --rm --name metrix metrix bash || true
	test -f /.dockerenv && ./run.sh

local: ## run local server
	docker-compose up -d db
	DB_HOST=localhost DB_PORT=5432 POSTGRES_USER=user POSTGRES_PASSWORD=stompy POSTGRES_DB=metrix ./run.sh

build: ## just build the app
	test -f /.dockerenv && cd metrix && cargo build

tests: ## run cargo tests
	test -f /.dockerenv || docker-compose build metrix
	test -f /.dockerenv || docker-compose run --service-ports --rm --name metrix metrix bash || true
	test -f /.dockerenv && ./test.sh

shell: ## jump into server container
	test -f /.dockerenv || docker exec -it metrix bash

migrate: ## run diesel_cli migrations
	cd /home/rocket/metrix; \
	test -f /usr/local/cargo/bin/diesel || cargo install diesel_cli ; \
	diesel setup --database-url="postgres://user:stompy@db/metrix"
