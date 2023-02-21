# OpenAPI Generator Petstore rust-server test

A project to test [OpenAPI Generator] capability of gnerating a petstore server in Rust (rust-server, Hyper based) and add a (Tower inspired) middleware to add CORS headers.

use `bin/generate_api.sh` script to generate the server stub from [openapi/petstore.yaml](openapi/petstore.yaml) OpenAPI 3.0 definition.

The server code is generated in the [server](server) directory.