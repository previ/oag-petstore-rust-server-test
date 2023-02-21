docker run --rm \
  -v ${PWD}:/local openapitools/openapi-generator-cli:latest generate \
  -i /local/openapi/petstore.yaml \
  -g rust-server \
  --generate-alias-as-model \
  -o /local/server 
