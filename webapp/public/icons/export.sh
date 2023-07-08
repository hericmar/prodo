#!/bin/sh

set -e

input="safari-pinned-tab.svg"
export inkscape="C:\\Program Files\\Inkscape\\bin\\inkscape"

names="apple-icon-120x120.png apple-icon-152x152.png apple-icon-167x167.png apple-icon-180x180.png favicon-16x16.png favicon-32x32.png favicon-96x96.png favicon-128x128.png icon-128x128.png icon-144x144.png icon-192x192.png icon-256x256.png icon-384x384.png icon-512x512.png ms-icon-144x144.png" 

for name in $names; do
  width=$(echo "$name" | sed 's/[^0-9]*\([0-9]*\).*/\1/')
  echo "Generating $name with width $width px."
  "$inkscape" \
    --export-width=$width \
    --export-type=png \
    --export-filename=$name \
    --export-background=0xffffff \
    $input
done
