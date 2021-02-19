default: lint

.PHONY: lint
lint:
	docker run -it --rm -v $$PWD:$$PWD -w $$PWD stoplight/spectral:master lint -s path-not-include-query definitions/v1.1/openapi.yaml
