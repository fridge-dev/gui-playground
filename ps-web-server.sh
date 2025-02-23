#!/usr/bin/env bash

set -euxo pipefail

ps -ef | grep "basic-http-server docs" | grep -v "grep"
