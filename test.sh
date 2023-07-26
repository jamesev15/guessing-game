#!/usr/bin/env bash
## Testing code

for DIR in */; do
    DIRNAME=$(basename "$DIR")
    echo "==> $DIRNAME <=="
    (cd $DIR && cargo test)
done

echo "Test completed."