#!/usr/bin/env bash
## Linting code

for DIR in */; do
    DIRNAME=$(basename "$DIR")
    echo "==> $DIRNAME <=="
    (cd $DIR && cargo clippy --all-targets --all-features -- -D warnings)
done

echo "Linting completed."