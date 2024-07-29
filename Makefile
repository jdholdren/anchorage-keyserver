.PHONY: check httpd migrate

ifneq (,${KEYSERVER_DOTENV})
    include ${KEYSERVER_DOTENV}
    export
endif

check:
	cargo check

# Alias for check
c: check

httpd:
	cargo run -- httpd

migrate:
	sqlx migrate run --database-url=sqlite://db.sqlite3
