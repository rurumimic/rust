FROM docker.io/library/rust:1.66-alpine

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

CMD ["app"]

