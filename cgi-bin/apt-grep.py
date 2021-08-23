#!/usr/bin/python3 -u

import cgi
import os
import subprocess

from pathlib import Path


def ripgrep(form):
    contents_index_dir = os.getenv(
        'APT_GREP_CONTENTS_INDEX_DIR',
        'contents_index_dir',
    )
    contents_indexes = []
    branches = form.getvalue('branches').split(',')
    arches = form.getvalue('arches').split(',')
    for branch in branches:
        for arch in arches:
            contents_index = Path(branch) / arch
            contents_indexes.append(contents_index.as_posix())

    re = form.getvalue('re')
    default_max_count = os.getenv('APT_GREP_DEFAULT_MAX_COUNT', 20)
    max_count = int(form.getvalue('max_count', default_max_count))

    cmd = ['rg', str(re)]
    if max_count > 0:
        cmd.append(f'--max-count={max_count}')
    if form.getvalue('filename') != "false":
        cmd.append('--with-filename')
    else:
        cmd.append("--no-filename")
    cmd.extend(contents_indexes)

    subprocess.run(cmd, cwd=contents_index_dir)


def main():
    form = cgi.FieldStorage()
    print('Content-Type: text/plain\n')
    ripgrep(form)


if __name__ == '__main__':
    main()
