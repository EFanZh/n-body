#!/bin/sh

COMMIT=$(git rev-parse HEAD)
COMMIT_MESSAGE=$(git show -s --format=%B "$COMMIT")

echo "# N-Body

Automatically generated from commit [$COMMIT](https://github.com/EFanZh/n-body/tree/$COMMIT)
([diff](https://github.com/EFanZh/n-body/commit/$COMMIT)).

Commit message:

"'```'"
$COMMIT_MESSAGE
"'```'
