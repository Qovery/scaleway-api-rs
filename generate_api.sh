#!/usr/bin/env bash

# this script is used to generate the whole project from open api specs provided by Scaleway.
set -e

rm -rf docs || true
rm -rf src || true
rm README.md || true

rm -rf .generation || true
mkdir .generation

# add open spec API file here
# =>
wget -P .generation/ https://developers.scaleway.com/static/726982498bc37f8a02c0cc356342e2aa/scaleway.instance.v1.Api.yml
wget -P .generation/ https://developers.scaleway.com/static/7f404aaae3cabd6b26f7e5f2a1e32e05/scaleway.registry.v1.Api.yml
wget -P .generation/ https://developers.scaleway.com/static/be99ddac31d6fc16d3797937da043821/scaleway.k8s.v1.Api.yml
wget -P .generation/ https://developers.scaleway.com/static/78dc39f6f39f212eff8813334e33fc65/scaleway.vpc.v1.Api.yml
wget -P .generation/ https://developers.scaleway.com/static/997fc131a47f74aeb8256967b0ba1e5a/scaleway.apple_silicon.v1alpha1.Api.yml
wget -P .generation/ https://developers.scaleway.com/static/124134235ef10c3bf8c87f813ab7a589/scaleway.flexible_ip.v1alpha1.Api.yml
wget -P .generation/ https://developers.scaleway.com/static/6eb7883c7e388fde62aa27c32de7b0b6/scaleway.domain.v2beta1.Api.yml

# merging specs into one file
cat << EOM > .generation/openapi-merge.json
{
  "inputs": [
    {"inputFile": "./scaleway.instance.v1.Api.yml"},
    {"inputFile": "./scaleway.registry.v1.Api.yml"},
    {"inputFile": "./scaleway.k8s.v1.Api.yml"},
    {"inputFile": "./scaleway.vpc.v1.Api.yml"},
    {"inputFile": "./scaleway.apple_silicon.v1alpha1.Api.yml"},
    {"inputFile": "./scaleway.flexible_ip.v1alpha1.Api.yml"},
    {"inputFile": "./scaleway.domain.v2beta1.Api.yml"}
  ],
  "output": "consolidated-specs.yml"
}
EOM

npx openapi-merge-cli --config .generation/openapi-merge.json

openapi-generator generate -g rust \
 -i .generation/consolidated-specs.yml \
 -o . \
 -c openapi-generator-config.yml \

# adding README elements
cat templates/README.prepend.md README.md > README.consolidated.md
mv README.consolidated.md README.md

# remove useless objects
rm -rf .generation
find . -type d -name '.openapi-generator' -exec rm -rf {} +

# cargo fmt and clippy
cargo fmt --all
cargo +nightly clippy --fix -Z unstable-options --allow-dirty --allow-staged