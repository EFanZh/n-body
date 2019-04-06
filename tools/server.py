#!/usr/bin/env python3

import functools
import http.server
import os


class _Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, directory=None, **kwargs):
        super().__init__(*args, directory=directory, **kwargs)

        self.extensions_map.update({
            '.js': 'application/javascript',
            '.wasm': 'application/wasm'
        })


def main():
    base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    handler_class = functools.partial(_Handler, directory=base_dir)

    http.server.test(HandlerClass=handler_class)


if __name__ == '__main__':
    main()
