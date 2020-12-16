#!/bin/bash

set -ex

ROOT_DIR=/data/local/tmp/cargo-android-sample
adb shell mkdir -p $ROOT_DIR

adb push $1 $ROOT_DIR

adb shell $ROOT_DIR/$(basename $1) ${@:2:$#}
