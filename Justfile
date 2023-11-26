

default:
    just -l

install:
    cargo install --path blua-cli


cross-build:
    cross build -p blua-cli --target x86_64-unknown-linux-musl --no-default-features --features luajit,vendored  --release
