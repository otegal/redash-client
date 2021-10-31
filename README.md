# redash-client

## setup local redash
cf. https://redash.io/help/open-source/dev-guide/docker

1. git clone that repository
2. exec `docker-compose up -d`
3. exec `docker-compose run --rm server create_db`
4. exec `yarn --frozen-lockfile`
5. exec `yarn build`
6. access http://localhost:5000/ when you face Redash Initial Setup page.

