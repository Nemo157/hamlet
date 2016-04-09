#!/bin/bash

set -e

: ${TRAVIS:?'This should only be run on Travis CI'}
: ${GITHUB_TOKEN:?'Must provide github token'}
REPO_SLUG=${1:?'Must provide repo slug'}

echo "machine github.com login $GITHUB_TOKEN password x-oauth-basic" >> ~/.netrc
chmod 0600 ~/.netrc
