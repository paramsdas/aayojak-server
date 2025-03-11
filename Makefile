server:
	cargo build --lib --manifest-path=./aayojak-server/Cargo.toml
	cargo test --manifest-path=./aayojak-server/Cargo.toml
	cargo doc --manifest-path=./aayojak-server/Cargo.toml

dev_db:
	docker image build -f ./Container/dev/db.Containerfile -t dev_db ./Container/
	docker container run -d -p 5432:5432 --rm --name dev_db -e POSTGRES_USER=test -e POSTGRES_PASSWORD=test -e POSTGRES_DB=aayojak dev_db
	docker container run -d -p 299:80 --rm --name pgadmin -e PGADMIN_DEFAULT_EMAIL=test@test.com -e PGADMIN_DEFAULT_PASSWORD=test dpage/pgadmin4

stop_dev_db:
	docker container stop dev_db pgadmin

dev_server:
	DATABASE_URL=postgres://test:test@host.docker.internal:5432/aayojak cargo run --manifest-path ./aayojak-server/Cargo.toml