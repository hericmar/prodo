#!/bin/sh

set -e

cd /usr/share/webapps/prodo

nginx
source /tmp/venv/bin/activate
exec gunicorn prodo.wsgi:application \
  --bind localhost:8000 \
  --workers 3
