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
wget -P .generation/ https://developers.scaleway.com/static/c78c1fb6947abf78efab65d4acc6dacd/scaleway.baremetal.v1.Api.yml
wget -P .generation/ https://developers.scaleway.com/static/805c4e0fa44967438ff07c28d87fac04/scaleway.rdb.v1.Api.yml 
wget -P .generation/ https://developers.scaleway.com/static/7f404aaae3cabd6b26f7e5f2a1e32e05/scaleway.registry.v1.Api.yml 
wget -P .generation/ https://developers.scaleway.com/static/72af768aeae6f8e82ef2ea61b4e46975/scaleway.k8s.v1.Api.yml 
wget -P .generation/ https://developers.scaleway.com/static/c6dfc13b6013041388d93a1f54f77abb/scaleway.vpc.v1.Api.yml 
wget -P .generation/ https://developers.scaleway.com/static/72a2c7b853875ff3d1470f3453d8cd9d/scaleway.iot.v1.Api.yml 
wget -P .generation/ https://developers.scaleway.com/static/0bc750daa510b57996c29662eb213213/scaleway.flexible_ip.v1alpha1.Api.yml 
wget -P .generation/ https://developers.scaleway.com/static/e68e360c068c64e2fabb3de44f988646/scaleway.domain.v2beta1.Api.yml 

# merging specs into one file
cat << EOM > .generation/openapi-merge.json
{
  "inputs": [
    {"inputFile": "./scaleway.baremetal.v1.Api.yml"},
    {"inputFile": "./scaleway.rdb.v1.Api.yml"},
    {"inputFile": "./scaleway.registry.v1.Api.yml"},
    {"inputFile": "./scaleway.k8s.v1.Api.yml"},
    {"inputFile": "./scaleway.vpc.v1.Api.yml"},
    {"inputFile": "./scaleway.iot.v1.Api.yml"},
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
 --type-mappings number=i64 \

# adding README elements
cat templates/README.prepend.md README.md > README.consolidated.md
mv README.consolidated.md README.md

# remove useless objects
rm -rf .generation
find . -type d -name '.openapi-generator' -exec rm -rf {} +

# cargo fmt and clippy
cargo fmt --all
cargo +nightly clippy --fix -Z unstable-options --allow-dirty --allow-staged
