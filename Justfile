

cross-build:
    cross build -p lua-cli --target x86_64-unknown-linux-musl --features vendored  --release
