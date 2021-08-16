#!/bin/sh -eu

content_index_dir="${APT_GREP_CONTENTS_INDEX_DIR-contents_index_dir}"
rsync_url=rsync://rsync.altlinux.org/ALTLinux

f_arches_for_branch() {
    branch=$1
    if [ "$branch" = p8 ]
    then
        echo noarch i586 x86_64
    else
        echo noarch i586 x86_64 armh aarch64 ppc64le
    fi
}

f_branches() {
    echo sisyphus p8 p9 p10
}

for branch in $(f_branches); do
    for arch in $(f_arches_for_branch "$branch"); do
        if [ "$branch" = Sisyphus ]; then
            url="$rsync_url/$branch/$arch"
        else
            url="$rsync_url/$branch/branch/$arch"
        fi
        out_dir="$content_index_dir/$branch"
        mkdir -p "$out_dir"
        rsync -Pt "$url/base/contents_index" "$out_dir/$arch"
    done
done
