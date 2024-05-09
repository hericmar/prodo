#!/bin/sh

set -e

echo $PWD $PRODO_VERSION

FILES="webapp/package.json webapp/dist/pwa/manifest.json"
PRODO_VERSION=${PRODO_VERSION:-$(git describe --tags --abbrev=0)}

for file in $FILES; do
  if [ -f $file ]; then
    echo "Bumping version in $file"
    sed -i 's/"version": "[^"]*"/"version": "${PRODO_VERSION}"/' $file
  fi
done
