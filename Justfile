#!/usr/bin/env -S just --justfile

# Display available commands
default:
  @just -f {{justfile()}} --list

# Build WebAssembly under `extension_lectio` and build extension
build:
  #!/usr/bin/env sh
  cd core && trunk build --release
  [[ $? -eq 0 ]] && cd ../extension && pnpm install && pnpm run build:isolate

# Clean all generated files
clean:
  #!/usr/bin/env sh
  cargo clean
  [[ -d target ]] && rm -rf target
  [[ -d extension/dist ]] && rm -rf extension/dist
  [[ -d extension/node_modules ]] && rm -rf extension/node_modules
  [[ -d core/dist ]] && rm -rf core/dist
