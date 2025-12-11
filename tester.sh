set -e 

cargo run -- --config test_center.conf
cargo run -- --config test_center2.conf

cargo run -- --function grid --grid-input test_center.png
cargo run -- --function grid --grid-input test_center2.png
