cargo build --release

samples record ./target/release/rray --output-file image.ppm --width 1100 --height 500
