#!/bin/bash
set -e

# Transfer binary to device
ROOT_DIR=/data/local/tmp/cargo-android-sample
adb shell mkdir -p $ROOT_DIR $ROOT_DIR/criterion
adb push $1 $ROOT_DIR

# Execute
adb shell CRITERION_HOME=$ROOT_DIR/criterion $ROOT_DIR/$(basename $1) ${@:2:$#}

# Transfer criterion report from device
rm -rf target/criterion_android
adb pull $ROOT_DIR/criterion target/criterion_android
