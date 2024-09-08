FROM postgres:16-alpine

EXPOSE 5432

USER postgres

COPY ./db_schema/* /docker-entrypoint-initdb.d/