echo Setup
wget https://musl.cc/aarch64-linux-musl-cross.tgz
tar xf aarch64-linux-musl-cross.tgz

export PATH=$PWD/aarch64-linux-musl-cross/bin:$PATH
