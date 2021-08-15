#!/bin/sh -eu

RE="$1"
BRANCHES="${2-Sisyphus}"
ARCHES="${3-x86_64}"
BRANCHES_LOWER="$(echo $BRANCHES | tr A-Z a-z)"

curl 127.0.0.1:8080 -G \
    --data-urlencode "branches=$BRANCHES_LOWER" \
    --data-urlencode "arches=$ARCHES" \
    --data-urlencode "add_noarch=true" \
    --data-urlencode "re=$RE" \
    #
