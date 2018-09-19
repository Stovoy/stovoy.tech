FROM debian:stretch-slim

WORKDIR /app
EXPOSE 80
ENTRYPOINT ["/app/stovoy-tech"]

ADD target/release/stovoy-tech /app/stovoy-tech
ADD static/dist /app/static
