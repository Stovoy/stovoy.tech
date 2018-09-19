#!/bin/bash -e

./build.sh

docker run -it \
    -v $(pwd)/static/dist:/app/static \
    -p 80:80 \
    stovoy.tech
