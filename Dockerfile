FROM nginx:1.17.1-alpine

RUN apk add --no-cache python3 py3-pip
RUN pip install --upgrade pip
