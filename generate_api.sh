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
wget -P .generation/ https://www.scaleway.com/en/developers/static/scaleway.baremetal.v1.Api.yml
wget -P .generation/ https://www.scaleway.com/en/developers/static/scaleway.rdb.v1.Api.yml
wget -P .generation/ https://www.scaleway.com/en/developers/static/scaleway.registry.v1.Api.yml
wget -P .generation/ https://www.scaleway.com/en/developers/static/scaleway.k8s.v1.Api.yml
wget -P .generation/ https://www.scaleway.com/en/developers/static/scaleway.vpc.v1.Api.yml
wget -P .generation/ https://www.scaleway.com/en/developers/static/scaleway.flexible_ip.v1alpha1.Api.yml
wget -P .generation/ https://www.scaleway.com/en/developers/static/scaleway.domain.v2beta1.Api.yml

# merging specs into one file
cat << EOM > .generation/openapi-merge.json
{
  "inputs": [
    {"inputFile": "./scaleway.baremetal.v1.Api.yml"},
    {"inputFile": "./scaleway.rdb.v1.Api.yml"},
    {"inputFile": "./scaleway.registry.v1.Api.yml"},
    {"inputFile": "./scaleway.k8s.v1.Api.yml"},
    {"inputFile": "./scaleway.vpc.v1.Api.yml"},
    {"inputFile": "./scaleway.flexible_ip.v1alpha1.Api.yml"},
    {"inputFile": "./scaleway.domain.v2beta1.Api.yml"}
  ],
  "output": "consolidated-specs.yml"
}
EOM

npx openapi-merge-cli --config .generation/openapi-merge.json

# remove x-enum-descriptions because openapi-generator-cli crash on it
yq -i --yaml-output 'walk(if type == "object" and has("x-enum-descriptions") then del(.["x-enum-descriptions"]) else . end)' .generation/consolidated-specs.yml

openapi-generator-cli generate -g rust \
 -i .generation/consolidated-specs.yml \
 -o . \
 -c openapi-generator-config.yml

# adding README elements
cat templates/README.prepend.md README.md > README.consolidated.md
mv README.consolidated.md README.md

# remove useless objects
find . -type d -name '.openapi-generator' -exec rm -rf {} +

# cargo fmt and clippy
cargo fmt --all
cargo clippy --fix --lib --all-features --allow-dirty --allow-staged