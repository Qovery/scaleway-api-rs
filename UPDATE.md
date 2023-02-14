# How to update generated code?

## Introduction
This library is auto-generated using [OpenAPI Generator](https://openapi-generator.tech).

## Dependencies
Here are dependencies you need to have to run generation locally:
- Rust tool chain (stable + nightly builds): https://www.rust-lang.org/tools/install
- `wget`: https://www.gnu.org/software/wget/
- `npm` and `npx`: https://www.npmjs.com & https://www.npmjs.com/package/npx
- `openapi-merge-cli`: https://www.npmjs.com/package/openapi-merge-cli
- `openapi-generator`: https://github.com/OpenAPITools/openapi-generator

## Adding / Updating / Deleting any API specs
Open specs links should be added into [generate_api.sh](generate_api.sh):
```
# add open spec API file here
# =>
wget -P .generation/ [YOUR_OPEN_SPEC_FILE] 
...
wget -P .generation/ https://developers.scaleway.com/static/55e3f254c8c01db735e2ee27ff81ad98/scaleway.baremetal.v1.Api.yml
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