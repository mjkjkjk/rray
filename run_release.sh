#!/bin/bash

cargo build --release

./target/release/rray --output-file image.ppm --depth 64 --samples 128

open image.ppm

