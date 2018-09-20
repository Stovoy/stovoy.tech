#!/bin/bash -e

nginx -c /app/nginx.conf &
exec /app/stovoy-tech
