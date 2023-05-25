#! /bin/bash
SCRIPT_DIR=$(cd -- $(dirname ${BASH_SOURCE[0]}) &> /dev/null && pwd)

if [[ $# -ne 2 ]]; then 
    printf "Illegal format. Format: debug.sh [TEST-NAME] [TEST-EXECUTABLE]" >&2
    exit 2
fi

# frees the debugging port
fuser -k 1234/tcp

cargo test --test $1 &

gdb $2