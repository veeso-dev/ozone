#!/bin/bash

set -e

export RUST_LOG="debug"

cargo make --profile production run
