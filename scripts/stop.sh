#!/bin/bash

set -e

cd $(dirname $0)/..
source scripts/.names.sh

docker stop $web_runner_name
docker stop $api_runner_name
