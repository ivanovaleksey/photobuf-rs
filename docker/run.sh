#! /bin/sh
set -xe

docker build -t photobuf -f docker/Dockerfile .
docker run -it --rm -v $(pwd)/target/x86_64-pc-windows-gnu:/build/target/x86_64-pc-windows-gnu photobuf
