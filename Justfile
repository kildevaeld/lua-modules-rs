

default:
    just -l

install:
    cargo install --path lua-cli


cross-build:
    cross build -p lua-cli --target x86_64-unknown-linux-musl --features vendored  --release
