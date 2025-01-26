#!/bin/bash

cargo run -- --output-file image.ppm --depth 4 --samples 4

open image.ppm
