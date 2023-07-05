#!/bin/sh

set -e

PRODO_USER=prodo

cd /usr/share/webapps/prodo
source venv/bin/activate

python manage.py $@

