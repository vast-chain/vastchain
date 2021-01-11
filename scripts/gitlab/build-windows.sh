#!/bin/bash
set -e # fail on any error
set -u # treat unset variables as error

set INCLUDE="C:\Program Files (x86)\Microsoft SDKs\Windows\v7.1A\Include;C:\vs2015\VC\include;C:\Program Files (x86)\Windows Kits\10\Include\10.0.10240.0\ucrt"
set LIB="C:\vs2015\VC\lib;C:\Program Files (x86)\Windows Kits\10\Lib\10.0.10240.0\ucrt\x64"
sccache -s

echo "__________Show ENVIROMENT__________"
echo "CI_SERVER_NAME:   " $CI_SERVER_NAME
echo "CARGO_HOME:       " $CARGO_HOME
echo "CARGO_TARGET:     " $CARGO_TARGET
echo "RUSTC_WRAPPER:    " $RUSTC_WRAPPER
echo "SCCACHE_DIR:      " $SCCACHE_DIR

echo "_____ Building target: "$CARGO_TARGET" _____"
  # NOTE: Enables the aes-ni instructions for RustCrypto dependency.
  # If you change this please remember to also update .cargo/config
export RUSTFLAGS=" -Ctarget-feature=+aes,+sse2,+ssse3 -Ctarget-feature=+crt-static"

time cargo build --target $CARGO_TARGET --verbose --release --features final
time cargo build --target $CARGO_TARGET --verbose --release -p evmbin
time cargo build --target $CARGO_TARGET --verbose --release -p vaststore-cli
time cargo build --target $CARGO_TARGET --verbose --release -p vastkey-cli

echo "__________Sign binaries__________"
scripts/gitlab/sign-win.cmd $keyfile $certpass target/$CARGO_TARGET/release/vast.exe
scripts/gitlab/sign-win.cmd $keyfile $certpass target/$CARGO_TARGET/release/vast-evm.exe
scripts/gitlab/sign-win.cmd $keyfile $certpass target/$CARGO_TARGET/release/vaststore.exe
scripts/gitlab/sign-win.cmd $keyfile $certpass target/$CARGO_TARGET/release/vastkey.exe

echo "_____ Post-processing binaries _____"
rm -rf artifacts
mkdir -p artifacts
cd artifacts
mkdir -p $CARGO_TARGET
cd $CARGO_TARGET
cp --verbose ../../target/$CARGO_TARGET/release/vast.exe ./vast.exe
cp --verbose ../../target/$CARGO_TARGET/release/vast-evm.exe ./vast-evm.exe
cp --verbose ../../target/$CARGO_TARGET/release/vaststore.exe ./vaststore.exe
cp --verbose ../../target/$CARGO_TARGET/release/vastkey.exe ./vastkey.exe

echo "_____ Calculating checksums _____"
for binary in $(ls)
do
  rhash --sha256 $binary -o $binary.sha256
  ./vast.exe tools hash $binary > $binary.sha3
done
cp vast.exe.sha256 vast.sha256
cp vast.exe.sha3 vast.sha3

sccache -s
