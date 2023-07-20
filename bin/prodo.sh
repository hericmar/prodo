#!/bin/sh

set -e

PRODO_USER=prodo

cd /usr/share/webapps/prodo
source venv/bin/activate

case $1 in
  start)
    echo "Starting Prodo..."
    exec gunicorn prodo.wsgi:application \
	    --bind localhost:9200 \
	    --workers 4 \
	    --capture-output \
	    --timeout 90 \
	    --pid /var/run/prodo.pid
    ;;
  *)
    python manage.py $@
    ;;
esac
