#!/bin/bash

set -e

pushd $(dirname $0)/..
source scripts/.names.sh

docker run --rm -d --runtime=io.containerd.spin.v2 --platform=wasi/wasm -p $api_port:80 --name $api_runner_name $api_image_name
docker run --rm -d -p $web_port:4200 --name $web_runner_name $web_image_name node_modules/\@angular/cli/bin/ng.js serve --host 0.0.0.0
docker logs -f $web_runner_name
