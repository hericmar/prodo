#!/bin/sh

export DJANGO_SUPERUSER_PASSWORD=prodo
export DJANGO_SUPERUSER_USERNAME=prodo

prodo migrate
prodo createsuperuser --noinput --email "noreply@example.com"

prodo start &

nginx -c /etc/nginx/nginx.conf
