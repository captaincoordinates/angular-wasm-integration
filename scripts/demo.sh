#!/bin/bash

pushd $(dirname $0)/..

scripts/build.sh
scripts/start.sh
echo
echo "stopping..."
scripts/stop.sh
