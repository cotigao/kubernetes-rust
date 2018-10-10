VERSION = 1.7.12

.PHONY: update-client
update-client:
	openapi-generator generate \
		-i https://raw.githubusercontent.com/kubernetes/kubernetes/v${VERSION}/api/openapi-spec/swagger.json \
		-g rust --output kubernetes \
		-DpackageName=kubernetes -DpackageVersion=${VERSION}
