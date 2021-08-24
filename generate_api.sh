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
wget -P .generation/ https://developers.scaleway.com/static/53eca7755870a063a29d172455c9a5a3/scaleway.baremetal.v1.Api.yml
wget -P .generation/ https://developers.scaleway.com/static/991ccc31ec600bedad34d7e8382799ec/scaleway.rdb.v1.Api.yml
wget -P .generation/ https://developers.scaleway.com/static/7f404aaae3cabd6b26f7e5f2a1e32e05/scaleway.registry.v1.Api.yml
wget -P .generation/ https://developers.scaleway.com/static/be99ddac31d6fc16d3797937da043821/scaleway.k8s.v1.Api.yml
wget -P .generation/ https://developers.scaleway.com/static/78dc39f6f39f212eff8813334e33fc65/scaleway.vpc.v1.Api.yml
wget -P .generation/ https://developers.scaleway.com/static/124134235ef10c3bf8c87f813ab7a589/scaleway.flexible_ip.v1alpha1.Api.yml
wget -P .generation/ https://developers.scaleway.com/static/89ef890736cb4e3d26984aaaf9c9b1e8/scaleway.domain.v2beta1.Api.yml

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