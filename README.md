# redash-client

## setup local redash
cf. https://redash.io/help/open-source/dev-guide/docker

1. git clone that repository.
2. exec `docker-compose up -d`
3. exec `docker-compose run --rm server create_db`
4. exec `yarn --frozen-lockfile`
5. exec `yarn build`
6. access http://localhost:5000/ when you face Redash Initial Setup page.
7. setup initial profile.
8. create PostgreSQL Data Source. refer redash/docker-compose.yml
9. create first query. `ex) SELECT * FROM users;`


if you want to permanent redash db, add `volumes` in `redash/docker-compose.yml`.
```Dockerfile
services:
  postgres:
    image: postgres:9.5-alpine
    # The following turns the DB into less durable, but gains significant performance improvements for the tests run (x3
    # improvement on my personal machine). We should consider moving this into a dedicated Docker Compose configuration for
    # tests.
    ports:
      - "15432:5432"
    command: "postgres -c fsync=off -c full_page_writes=off -c synchronous_commit=OFF"
    restart: unless-stopped
    environment:
      POSTGRES_HOST_AUTH_METHOD: "trust"
    volumes:
      - redash-pg-local-data:/var/lib/postgresql/data

volumes:
  redash-pg-local-data:
    driver: local
```
