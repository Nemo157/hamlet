#!/bin/bash

set -e

TAG=${1:?'Missing tag to publish for'}
TOKEN=${2:?'Missing crates.io token to publish with'}

if ! grep -Fxq "version = "'"'"$TAG"'"' Cargo.toml
then
  echo "Tag $TAG doesn't appear to be the version in Cargo.toml"
fi

cargo publish --verbose --token "$TOKEN"
