#!/bin/bash

set -e

export RUST_LOG="info"

cargo make --profile production run
