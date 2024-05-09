FROM alpine:edge

WORKDIR /app

RUN apk update && apk add --no-cache \
      gcc \
      git \
      libffi-dev \
      libpq-dev \
      make \
      musl-dev \
      nginx \
      python3 \
      python3-dev && \
    apk add --repository=https://dl-cdn.alpinelinux.org/alpine/edge/testing pnpm

COPY . /app
COPY ./docker/entrypoint.sh /app/entrypoint.sh
COPY ./etc/nginx/nginx.conf /etc/nginx/nginx.conf

RUN make && make install && \
    source /usr/share/webapps/prodo/venv/bin/activate && pip install psycopg2 && deactivate && \
    apk del gcc git libffi-dev make musl-dev python3-dev
