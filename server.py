#!/usr/bin/env python3

import http.server


class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, directory=None, **kwargs):
        super().__init__(*args, directory=directory, **kwargs)

        self.extensions_map.update({
            '.js': 'application/javascript',
            '.wasm': 'application/wasm'
        })


def main():
    http.server.test(HandlerClass=Handler)


if __name__ == '__main__':
    main()
