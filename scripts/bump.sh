#!/bin/sh

version=$(sed -n '/^version[[:space:]]*=/s/^[^"]*"\([^"]*\)".*$/\1/p' Cargo.toml)

major=$(echo "$version" | cut -d. -f1)
minor=$(echo "$version" | cut -d. -f2)
patch=$(echo "$version" | cut -d. -f3)

patch=$((patch + 1))

version="${major}.${minor}.${patch}"

echo "New version: $version"
sed "s/^version[[:space:]]*=.*$/version = \"$version\"/" Cargo.toml > Cargo.toml.new
mv Cargo.toml.new Cargo.toml
