#!/bin/sh -eu

RE="$1"
BRANCHES="${2-Sisyphus}"
ARCHES="${3-x86_64}"
LINES="${4-20}"
LINES_IN_FILE="${5-20}"
BRANCHES_LOWER="$(echo $BRANCHES | tr A-Z a-z)"

curl 127.0.0.1:8080 -G \
    --data-urlencode "branches=$BRANCHES_LOWER" \
    --data-urlencode "arches=$ARCHES" \
    --data-urlencode "lines=$LINES" \
    --data-urlencode "lines_in_file=$LINES_IN_FILE" \
    --data-urlencode "add_noarch=true" \
    --data-urlencode "re=$RE" \
    --data-urlencode "filename=false" \
    #
