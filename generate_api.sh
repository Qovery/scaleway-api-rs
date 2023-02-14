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
  https://developers.scaleway.com/static/50fa035ee346239a0081bd35a9a42216/scaleway.instance.v1.Api.yml
  https://developers.scaleway.com/static/ea0a7cfc643ebb19d0aa7e513239d0f8/scaleway.baremetal.v1.Api.yml
  https://developers.scaleway.com/static/73f07c9471203f4a936ccd1c732336e6/scaleway.registry.v1.Api.yml
  https://developers.scaleway.com/static/fdb3fd2472c3cd704cbb68ee1dcce3aa/scaleway.rdb.v1.Api.yml
  https://developers.scaleway.com/static/c6a9ddee34be6ead98f400fd75bf09ae/scaleway.k8s.v1.Api.yml
  https://developers.scaleway.com/static/b308ccdf3939490704988c9c0356c15b/scaleway.vpc.v1.Api.yml
  https://developers.scaleway.com/static/c347b9a9ee7e44303ec37e6b438def45/scaleway.domain.v2beta1.Api.yml
  https://developers.scaleway.com/static/9b33dabdafcbaaef7f079d1b0c51f887/scaleway.iam.v1alpha1.Api.yml
  https://developers.scaleway.com/static/e10e8c13cd8bd9b49d61331dace3662a/scaleway.account.v2.Api.yml
  https://developers.scaleway.com/static/59091ac98ce80e916aa85086412be11e/scaleway.billing.v2alpha1.Api.yml
  https://developers.scaleway.com/static/ca958b69824091806259ae71d1cf0a23/scaleway.flexible_ip.v1alpha1.Api.yml
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
rm -rf .generation
find . -type d -name '.openapi-generator' -exec rm -rf {} +

# cargo fmt and clippy
cargo fmt --all
cargo +nightly clippy --fix -Z unstable-options --allow-dirty --allow-staged
