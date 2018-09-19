#!/bin/bash -e

docker tag stovoy.tech stovoy/stovoy.tech
docker push stovoy/stovoy.tech

ssh stovoy.tech 'docker pull stovoy/stovoy.tech &&
    ( docker rm -f stovoy.tech > /dev/null 2>&1 || true ) &&
    docker run \
        --publish 80:80 \
        --publish 443:443 \
        --name stovoy.tech \
        --volume /etc/letsencrypt:/ssl \
        --detach \
        stovoy/stovoy.tech'
