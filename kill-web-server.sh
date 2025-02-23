#!/usr/bin/env bash

set -euxo pipefail

kill -9 $(ps -ef | grep "basic-http-server docs" | grep -v "grep" | awk '{print $2}')
