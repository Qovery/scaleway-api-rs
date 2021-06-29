# How to update generated code?

## Introduction
This library is auto-generated using [OpenAPI Generator](https://openapi-generator.tech).

## Adding / Updating / Deleting any API specs
Open specs links should be added into [generate_api.sh](generate_api.sh):
```
# add open spec API file here
# =>
wget -P .generation/ [YOUR_OPEN_SPEC_FILE] 
...
wget -P .generation/ https://developers.scaleway.com/static/55e3f254c8c01db735e2ee27ff81ad98/scaleway.baremetal.v1.Api.yml
```
Then add your downloaded file into the input for concatenation (open API generator needs to have only one specs file).
```
# merging specs into one file
cat << EOM > .generation/openapi-merge.json
{
  "inputs": [
    {"inputFile": "./[YOUR_OPEN_SPEC_FILE]"},
    ...
    {"inputFile": "./scaleway.domain.v2beta1.Api.yml"}
  ],
  "output": "consolidated-specs.yml"
}
EOM
```
Then it should be ok, you can place yourself at project root and launch the generation script:
```
$ ./generate_api.sh
...
################################################################################
# Thanks for using OpenAPI Generator.                                          #
# Please consider donation to help us maintain this project 🙏                 #
# https://opencollective.com/openapi_generator/donate                          #
################################################################################
```

We should also maybe consider to bump the package version using [semver](https://semver.org/).