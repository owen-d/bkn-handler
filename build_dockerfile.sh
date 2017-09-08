#!/bin/bash
set -e

REGISTRY_HOST=gcr.io
PROJECT_ID=glassy-courage-146901
IMAGE=bkn-handler-rs
DOCKERFILE_DIR=${DOCKERFILE_DIR:-.}
DOCKERFILE_LOCATION=$DOCKERFILE_DIR/Dockerfile
PREFIX=$REGISTRY_HOST/$PROJECT_ID/$IMAGE

# open fd for redirecting subrouting stdout to current shell stdout
exec 6>&1

# use ekidd's rust-musl-builder:
# https://github.com/emk/rust-musl-builder
docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder:nightly cargo build --release

IMAGE_ID=$(docker build $DOCKERFILE_DIR | tee >(cat - >&6) | tail -n 1 | awk '{print $3}')
TAG=$PREFIX:$IMAGE_ID

# close fd
exec 6>&-

echo -e "\n\n\n\n..........\nusing image-id: $IMAGE_ID as tag.\nFull image: $TAG\n.........."

docker tag $IMAGE_ID $TAG

# docker push $TAG
