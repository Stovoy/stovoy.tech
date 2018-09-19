FROM debian:stretch-slim

WORKDIR /app
EXPOSE 80 443
ENTRYPOINT ["/app/stovoy-tech"]

RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/cache
ADD target/release/stovoy-tech /app/stovoy-tech
ADD static/dist /app/static
