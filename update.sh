#!/usr/bin/env bash

set -euxo pipefail

rm -rf src
mkdir src

#(cd ../chiptool/; cargo build)
chiptool generate --svd svd/psoc4100s.svd --transform svd/psoc4100s.yaml

# cargo install form
form -i lib.rs -o src
rm lib.rs

cargo fmt
sed -i '/#!\[doc =/c\#![doc = include_str!("../README.md")]\n#![allow(non_camel_case_types)]' src/lib.rs

cargo check
