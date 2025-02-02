#!/bin/bash

cargo build --release

./target/release/rray --output-file image.ppm --depth 64 --samples 128 --width 800 --height 450

open image.ppm

