.DEFAULT_GOAL:=help
.PHONY: help server

help: ## Show all the available make commands
	@echo "======================================================================================================================================================================="
	@awk '/```ascii/{a=1; next}/```/{a=0}(a==1){print}' README.md
	@echo "=======================================================================================================================================================================\n\n"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

server: ## start docker/server
	test -f /.dockerenv || docker-compose build metrix
	test -f /.dockerenv || docker-compose run --service-ports --rm --name metrix metrix bash || true
	test -f /.dockerenv && cd metrix && cargo run

shell: ## jump into server container
	test -f /.dockerenv || docker exec -it metrix bash
