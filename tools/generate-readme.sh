#!/bin/sh

COMMIT=$(git rev-parse HEAD)

echo "# N-Body

Automatically generated from commit [$COMMIT](https://github.com/EFanZh/n-body/tree/$COMMIT)."
