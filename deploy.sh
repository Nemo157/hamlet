#!/bin/bash

set -e

: ${CRATES_IO_TOKEN:?'Missing crates.io token to publish with'}
TAG=${1:?'Missing tag to publish for'}

if ! grep -Fxq "version = "'"'"$TAG"'"' Cargo.toml
then
  echo "Tag $TAG doesn't appear to be the version in Cargo.toml"
fi

cargo publish --verbose --token "$CRATES_IO_TOKEN"

# Setup this repo to publish the docs
git fetch origin gh-pages
git checkout -b gh-pages FETCH_HEAD

# Move the built docs into versioned folder
mv target/doc docs/$TAG

# Update the index to point to the versioned docs
sed -i'' -e '/<!-- TRAVIS: PUT THE DOCS HERE-->/a\
<li><a href="docs/'"$TAG"'/hamlet/">'"$TAG"'</a></li>
' index.html

# Add the changes
git add docs/$TAG
git add index.html

# Commit and push
git commit -m "Add API docs for $TAG"
git push origin gh-pages:gh-pages
