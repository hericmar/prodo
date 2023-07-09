#!/bin/sh

set -e

export version=$(git describe --tags --abbrev=0)

manifest=$(jq --arg version $version '.version = $version' webapp/dist/pwa/manifest.json)

echo "$manifest" > webapp/dist/pwa/manifest.json

