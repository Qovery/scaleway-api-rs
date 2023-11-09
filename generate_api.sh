#!/usr/bin/env bash

# this script is used to generate the whole project from open api specs provided by Scaleway.
set -e

function join_by {
  local d=${1-} f=${2-}
  if shift 2; then
    printf %s "$f" "${@/#/$d}"
  fi
}

rm -rf docs || true
rm -rf src || true
rm README.md || true

rm -rf .generation || true
mkdir .generation

# add open spec API file here
# =>
apis=(
  https://www.scaleway.com/en/developers/static/scaleway.instance.v1.Api.yml
  https://www.scaleway.com/en/developers/static/scaleway.baremetal.v1.Api.yml
  https://www.scaleway.com/en/developers/static/scaleway.registry.v1.Api.yml
  https://www.scaleway.com/en/developers/static/scaleway.rdb.v1.Api.yml
  https://www.scaleway.com/en/developers/static/scaleway.k8s.v1.Api.yml
  https://www.scaleway.com/en/developers/static/scaleway.vpc.v2.Api.yml
  https://www.scaleway.com/en/developers/static/scaleway.domain.v2beta1.Api.yml
  https://www.scaleway.com/en/developers/static/scaleway.iam.v1alpha1.Api.yml
  https://www.scaleway.com/en/developers/static/scaleway.account.v2.Api.yml
  https://www.scaleway.com/en/developers/static/scaleway.billing.v2alpha1.Api.yml
  https://www.scaleway.com/en/developers/static/scaleway.flexible_ip.v1alpha1.Api.yml
)
for i in "${apis[@]}"; do
  wget -P .generation/ "$i"
done

# merging specs into one file
json_files=$(ls .generation/ | tr '\n' ' ')
json_array=( $json_files )
inputs=$(join_by '"},{"inputFile": "./' "${json_array[@]}")
inputs="{\"inputFile\": \"./${inputs}\"}"
cat << EOM > .generation/openapi-merge.json
{
  "inputs": [
    $inputs
  ],
  "output": "consolidated-specs.yml"
}
EOM

npx openapi-merge-cli --config .generation/openapi-merge.json

openapi-generator-cli generate -g rust \
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
