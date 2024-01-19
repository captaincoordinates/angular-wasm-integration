#!/bin/bash

export api_image_name=angwasm-api
export processor_image_name=angwasm-processor-builder
export web_image_name=angwasm-web

export api_runner_name=$api_image_name-runner
export web_runner_name=$web_image_name-runner

export api_port=8080
export web_port=8123