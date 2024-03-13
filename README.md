Run exporter:
> cargo run --bin exporter

Ensure that `export.yaml` file exists and is properly formatted.

Build game with:
> cargo build -p game --release --target=wasm32-unknown-unknown

./gccl console bundle -c ./../../../3d_next/target/wasm32-unknown-unknown/release/game.wasm -a ./../../../3d_next/5r5g4b_color.gce