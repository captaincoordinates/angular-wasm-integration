#!/bin/bash

set -e

pushd $(dirname $0)/..
source scripts/.names.sh

echo "Docker WASM functionality must be enabled"

echo "building API"
docker build \
    -f dockerfile.api \
    -t $api_image_name \
    --platform wasi/wasm \
    .

echo "building processor"
docker build \
    -f dockerfile.processor \
    -t $processor_image_name \
    .
docker run \
    --rm \
    -v $PWD/image-processor:/image-processor:rw \
    -e API_BASE=http://localhost:$api_port \
    $processor_image_name \
    wasm-pack build /image-processor

echo "building web"
docker build \
    -f dockerfile.web \
    -t $web_image_name \
    .
