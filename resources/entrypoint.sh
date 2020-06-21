#!/bin/bash -e

start_nginx() {
    (
        echo "Launching Nginx"
        nginx -c /app/nginx.conf &
        PID=$!

        while true
        do
            sleep 0.1
            if ps -p "$PID" > /dev/null; then
                continue
            fi

            echo "Relaunching Nginx"
            nginx -c /app/nginx.conf &
            PID=$!
        done
    ) &
}

start_nginx
exec /app/stovoy-tech
