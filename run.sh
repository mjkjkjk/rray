#!/bin/bash

cargo run -- --output-file image.ppm --depth 4 --samples 4 --width 700 --height 400

open image.ppm
