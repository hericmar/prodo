FROM alpine:edge as rust-builder

WORKDIR /opt/prodo

RUN apk add --no-cache cargo musl-dev postgresql16-dev rust

COPY migrations /opt/prodo/migrations
COPY src /opt/prodo/src
COPY Cargo.toml Cargo.lock /opt/prodo/

RUN cargo build --release

FROM alpine:edge as node-builder

WORKDIR /opt/prodo/webapp

RUN apk add nodejs pnpm

COPY webapp /opt/prodo/webapp

RUN pnpm install && pnpm run build --mode pwa

FROM alpine:edge

RUN apk add --no-cache libpq libstdc++ musl-dev

COPY --from=rust-builder /opt/prodo/target/release/prodo /usr/bin/prodo
COPY --from=node-builder /opt/prodo/webapp/dist/* /usr/share/webapps/prodo/static

CMD ["prodo", "start"]
