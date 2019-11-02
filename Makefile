server:
	test -f /.dockerenv || docker-compose build metrix
	test -f /.dockerenv || docker-compose run --rm metrix bash
	test -f /.dockerenv && cd metrix && cargo run
