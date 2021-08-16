#!/usr/bin/python3

import argparse
import platform
import subprocess

from pathlib import Path

import yaml


def get_defaults():
    return {
        'url':  '127.0.0.1:8080',
        'branches': ['sisyphus'],
        'arches': [platform.machine()],
        'lines': 20,
        'add_noarch': True,
        'filename': None,
    }


def get_config(defaults):
    config_yaml = Path('/etc/apt-grep/client.yaml')
    if not config_yaml.exists():
        return defaults

    with open(config_yaml) as f:
        config = yaml.safe_load(f)
        if config is None:
            return defaults
        for k, v in defaults.items():
            if k not in config:
                config[k] = defaults[k]

    return config


def parse_args(defaults):
    parser = argparse.ArgumentParser(
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser.add_argument(
        '-u',
        '--url',
        default=defaults['url'],
        help='url to apt-grep server',
    )
    parser.add_argument(
        '-b',
        '--branches',
        nargs='+',
        default=defaults['branches'],
        help='list of branches',
    )
    parser.add_argument(
        '-a',
        '--arches',
        nargs='+',
        default=defaults['arches'],
        help='list of arches',
    )
    parser.add_argument(
        '-l',
        '--lines',
        default=defaults['lines'],
        help='limit lines in output',
    )
    parser.add_argument(
        '--add-noarch',
        action='store_true',
        dest='add_noarch',
        default=defaults['add_noarch'],
        help='add noarch to list of arches',
    )
    parser.add_argument(
        '--no-add-noarch',
        action='store_false',
        dest='add_noarch',
        default=defaults['add_noarch'],
        help='do not add noarch to list of arches',
    )
    parser.add_argument(
        '-f',
        '--filename',
        action='store_true',
        default=defaults['filename'],
        help='show branch and arch info',
    )
    parser.add_argument(
        'regexp',
        help='searching regexp',
    )
    args = parser.parse_args()

    if args.filename is None:
        if len(args.branches) > 1 or len(args.arches) > 1:
            args.filename = True
        else:
            args.filename = False

    return args


def do_request(args):
    cmd = [
        'curl',
        args.url,
        '-G',
        '--data-urlencode', f"branches={','.join(args.branches)}",
        '--data-urlencode', f"arches={','.join(args.arches)}",
        '--data-urlencode', f"lines={args.lines}",
        '--data-urlencode', f"add_noarch={str(args.add_noarch).lower()}",
        '--data-urlencode', f"filename={str(args.filename).lower()}",
        '--data-urlencode', f"re={args.regexp}",
    ]
    subprocess.run(cmd)


def main():
    defaults = get_defaults()
    config = get_config(defaults)
    args = parse_args(config)
    do_request(args)


if __name__ == '__main__':
    main()
