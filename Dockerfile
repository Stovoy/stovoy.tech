FROM debian:stretch-slim
ADD target/release/stovoy-tech /stovoy-tech
EXPOSE 80
ENTRYPOINT ["/stovoy-tech"]
