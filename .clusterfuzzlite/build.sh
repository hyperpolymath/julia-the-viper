#!/bin/bash -eu

cd "$SRC"/project
cargo +nightly fuzz build --release
find fuzz/target -maxdepth 4 -name 'fuzz_*' -executable \
  -not -name '*.d' -exec cp {} "$OUT"/ \;
