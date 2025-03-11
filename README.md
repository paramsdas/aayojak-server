# Aayojak Server

Aayojak is a basic todo-tool written in Rust. It uses Postgres as its database. This repository holds the code for Aayojak's backend.

## Local development

### Requirements

- [make](https://www.gnu.org/software/make/)
- [docker](https://docs.docker.com/)

### Steps

#### Startup

1. Navigate to the root directory
2. Run `make dev_db`
3. Run `make dev_server` -> blocks terminal
4. API should now be accessible at [localhost:8080](http://localhost:8080)

#### Cleanup

1. Exit the terminal where the dev server is running
2. Run `make stop_dev_db`

#### Testing the database connection

As part of the dev_db a container with [pgadmin](https://www.pgadmin.org/) is also installed as a container and exposed to the localhost at port 299. To access pgadmin, follow the steps below:

1. Open [localhost:299](http://localhost:299)
2. login using the following credentials
   - email: test@test.com
   - password: test
3. Click on 'Add Server'
4. Choose any name you'd like in the 'General' tab (e.g. aayojak)
5. Switch to the 'Connection' tab and fill out the following details:
   - host: host.docker.internal
   - password: test
6. The connection to the database running in the dev_db container should now be established.

NOTE: The container also exposes the database to the localhost at port 5432, in case you'd like to use a local installation of pgadmin

# Open Todos

### Improve Testing

- Unit tests
- Integration tests

### Interlinking Todos

- Create a model to interlink todos
- Implement the model
