default: lint gen examples

.PHONY: tools
tools:
	npm install -g @apidevtools/swagger-cli
	go install github.com/deepmap/oapi-codegen/cmd/oapi-codegen@v1.5.1


.PHONY: lint
lint:
	docker run -it --rm -v $$PWD:$$PWD -w $$PWD stoplight/spectral:master lint -s path-not-include-query -s oas3-unused-components-schema src/v1.1/root.yaml

.PHONY: lint_all
lint_all:
	docker run -it --rm -v $$PWD:$$PWD -w $$PWD stoplight/spectral:master lint -s path-not-include-query -s oas3-unused-components-schema {src,definitions}/v1.1/*.{json,yaml,yml}

.PHONY: gen _gen
gen: _gen lint_all

_gen:
	swagger-cli bundle src/v1.1/root.yaml -o definitions/v1.1/openapi.yaml --type yaml
	swagger-cli bundle src/v1.1/root.yaml -o definitions/v1.1/openapi.json --type json

.PHONY: examples
examples:
	# docker run -it --rm -v $$PWD:$$PWD -w $$PWD openapitools/openapi-generator-cli generate -i definitions/v1.1/openapi.yaml -g rust -o examples/rust/
	oapi-codegen -generate types,client,spec -package sacloud definitions/v1.1/openapi.yaml  > examples/go/sacloud/gen.go
