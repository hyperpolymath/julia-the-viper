#!/bin/bash -eu

cd "$SRC"/project

SANITIZER="${SANITIZER:-address}"

if [ "$SANITIZER" = "address" ]; then
  cargo +nightly fuzz build --release
elif [ "$SANITIZER" = "undefined" ]; then
  # cargo fuzz defaults to ASan; for UBSan we suppress that and inject UBSan
  # via RUSTFLAGS so the bad_build_check does not flag a sanitizer mismatch.
  RUSTFLAGS="-Z sanitizer=undefined" \
    cargo +nightly fuzz build --sanitizer none --release
else
  cargo +nightly fuzz build --release
fi

find fuzz/target -maxdepth 4 -name 'fuzz_*' -executable \
  -not -name '*.d' -exec cp {} "$OUT"/ \;
