echo Setup
wget https://musl.cc/aarch64-linux-musl-cross.tgz
tar xf aarch64-linux-musl-cross.tgz

export PATH=$PWD/aarch64-linux-musl-cross/bin:$PATH
cargo build  --target=aarch64-unknown-linux-musl
mkdir bin/
cp target/aarch64-unknown-linux-musl/debug/snap_api bin/
