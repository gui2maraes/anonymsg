#!/usr/bin/env bash

# generate JSON string of RSA key

openssl genrsa -out private.pem 2048
openssl rsa -in private.pem -pubout | awk -v ORS='\\n' '1' - > public.pem
