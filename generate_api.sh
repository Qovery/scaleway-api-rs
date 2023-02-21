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
  https://developers.scaleway.com/static/abea1f03df43785d42d43fb613494c7d/scaleway.instance.v1.Api.yml
  https://developers.scaleway.com/static/b4f0df3880468cf8ec7b4d67a3b094b2/scaleway.baremetal.v1.Api.yml
  https://developers.scaleway.com/static/41774d1984ba771f55939f80b75ebafb/scaleway.registry.v1.Api.yml
  https://developers.scaleway.com/static/77d5770fdbf691ac3b8e01c302c8aba4/scaleway.rdb.v1.Api.yml
  https://developers.scaleway.com/static/64746a0346bf0d5fd9ea237f2a875031/scaleway.k8s.v1.Api.yml
  https://developers.scaleway.com/static/1be1e71800c33373cec0aae9f44d843a/scaleway.vpc.v1.Api.yml
  https://developers.scaleway.com/static/8318de7a5ffda3e04eeaa012e22eada9/scaleway.domain.v2beta1.Api.yml
  https://developers.scaleway.com/static/40454d8c524dbc4f6fb33148e2e10087/scaleway.iam.v1alpha1.Api.yml
  https://developers.scaleway.com/static/ef71d5e750dfa42e203cd95867121062/scaleway.account.v2.Api.yml
  https://developers.scaleway.com/static/2fa5e51f29d14b3c316e09c3d8f0724d/scaleway.billing.v2alpha1.Api.yml
  https://developers.scaleway.com/static/58c95122aec43b8fa90b23c71fd5d15f/scaleway.flexible_ip.v1alpha1.Api.yml
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

openapi-generator generate -g rust \
 -i .generation/consolidated-specs.yml \
 -o . \
 -c openapi-generator-config.yml \
 --type-mappings number=i64 \

# adding README elements
cat templates/README.prepend.md README.md > README.consolidated.md
mv README.consolidated.md README.md

# remove useless objects
# rm -rf .generation (keep it because Scaleway remove them so we can't 
find . -type d -name '.openapi-generator' -exec rm -rf {} +

# cargo fmt and clippy
cargo fmt --all
cargo +nightly clippy --fix -Z unstable-options --allow-dirty --allow-staged
