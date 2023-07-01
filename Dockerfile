FROM alpine:latest

COPY requirements.txt /tmp/requirements.txt

WORKDIR /usr/share/webapps/prodo

RUN apk add --no-cache \
        nginx \
        python3 \
        py3-pip \
        wget
RUN wget -qO- https://get.pnpm.io/install.sh | ENV="$HOME/.shrc" SHELL="$(which sh)" sh -
RUN python3 -m venv /tmp/venv && \
    source /tmp/venv/bin/activate && \
    pip3 install --upgrade pip && \
    pip3 install -r /tmp/requirements.txt

COPY docker/bin /usr/bin

ENTRYPOINT ["sh", "/usr/bin/entrypoint.sh"]
