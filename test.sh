#!/bin/bash

INPUT="foo
baz
foofoo"

SED_RUST="./target/release/sed-rust"

TEST_CASES=(
    "s/foo/bar/"    # Replace first occurrence per line
    "s/foo/bar/g"   # Replace all occurrences per line
    "2,3s/foo/bar/" # Replace on lines 2 to 3
    "1,2s/foo/bar/" # Replace only at line 2
    "3,2s/foo/bar/" # Replace only the highest line (max(start, end))
)

exit_code=0

for TEST in "${TEST_CASES[@]}"; do
    echo "Testing: $TEST"    
    echo "$INPUT" | "$SED_RUST" "$TEST" > rust_output.txt
    echo "$INPUT" | sed "$TEST" > sed_output.txt
    if diff rust_output.txt sed_output.txt > /dev/null; then
        echo "✅ PASSED"
    else
        echo "❌ FAILED"
        echo "Expected:"
        cat sed_output.txt
        echo "Got:"
        cat rust_output.txt
        exit_code=1
    fi
    echo "-----------------------------"
done

rm rust_output.txt sed_output.txt
exit $exit_code
