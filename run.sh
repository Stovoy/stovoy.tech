#!/bin/bash -e

./build.sh

docker run -it -p 80:80 stovoy.tech
