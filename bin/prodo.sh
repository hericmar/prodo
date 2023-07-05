#!/bin/sh

set -e

PRODO_USER=prodo

cd /usr/share/webapps/prodo
source venv/bin/activate

case $1 in
  start)
    echo "Starting Prodo..."
    gunicorn prodo.wsgi:application --bind
    ;;
  *)
    python manage.py $@
    ;;
esac
