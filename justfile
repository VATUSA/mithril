all:

prepare-sql:
    DATABASE_URL="mysql://root:password@localhost:3306/combined?ssl-mode=disabled" cargo sqlx prepare

docker-build:
    docker build -t localhost/mithril .

docker-run:
    docker run --rm -p 4000:4000 -e DATABASE_URL_VATUSA="mysql://root:password@mysql:3306/combined?ssl-mode=disabled" -e DATABASE_URL_COBALT="mysql://root:password@mysql:3306/combined?ssl-mode=disabled" --network mithril_default localhost/mithril:latest
