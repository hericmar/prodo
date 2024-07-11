#!/bin/sh

set -e

JSON_FILES="webapp/package.json webapp/dist/pwa/manifest.json"
PRODO_VERSION=${PRODO_VERSION:-$(git describe --tags --abbrev=0)}
PRODO_VERSION=${PRODO_VERSION:1}

for file in $JSON_FILES; do
  if [ -f $file ]; then
    echo "Bumping version in $file"
    sed -i 's/"version":\s*"[^"]*"/"version": "'${PRODO_VERSION}'"/' $file
  fi
done

echo "Bumping package version in Cargo.toml"
sed -i 's/^version = "[^"]*"/version = "'${PRODO_VERSION}'"/' Cargo.toml
