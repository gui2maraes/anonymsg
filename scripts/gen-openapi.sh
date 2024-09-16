#!/usr/bin/env bash

set -x
set -eo pipefail

if ! [ -x "$(command -v openapi-generator-cli)" ]; then
    echo >&2 "Error: openapi-generator is not installed."
    exit 1
fi

if [ -z "$1" ]; then
    echo "Please supply path to openapi.yaml"
    exit 1
fi


openapi-generator-cli generate -i "$1" -g typescript-fetch -o web/src/api \
    --additional-properties=supportsES6=true