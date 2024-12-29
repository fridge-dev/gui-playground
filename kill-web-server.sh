#!/usr/bin/env bash

set -euxo pipefail

kill -9 $(ps -ef | grep "basic-http-server web" | grep -v "grep" | awk '{print $2}')
