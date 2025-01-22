#!/usr/bin/env bash
cargo release version $(./release-package-args.sh) --execute $1
