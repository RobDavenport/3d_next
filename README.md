Run exporter:
> cargo run --bin exporter

Ensure that `export.yaml` file exists and is properly formatted.

```yaml
# File Setup
# Input Directory containing all files
in_dir: "assets"

# Output Directory for the exported data
out_dir: "game/src/generated"

# Optional modfile which contains the exported assets
out_file: "mod.rs"

# Lists of Assets
# Images
images:
- tex1
- tex2
- tex3

# Meshes in .glb format to export
meshes:
- mesh1
- mesh2
- mesh3
```

Build game with:
> cargo build -p game --release --target=wasm32-unknown-unknown

# Run in Release
./gccl console bundle -c ./../../../3d_next/target/wasm32-unknown-unknown/release/game.wasm -a ./../../../3d_next/5r5g4b_color.gce


# Perf/Analysis
./gccl console bundle -c ./../../../3d_next/target/wasm32-unknown-unknown/perf/game.wasm -a ./../../../3d_next/5r5g4b_color.gce