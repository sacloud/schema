package sacloud

import (
	"os"

	"github.com/deepmap/oapi-codegen/pkg/securityprovider"
)

const APIRootURL = "https://secure.sakura.ad.jp/cloud/zone/is1a/api/cloud/1.1"

func NewClientFromEnv() (*Client, error) {
	basicAuthProvider, err := securityprovider.NewSecurityProviderBasicAuth(
		os.Getenv("SAKURACLOUD_ACCESS_TOKEN"),
		os.Getenv("SAKURACLOUD_ACCESS_TOKEN_SECRET"),
	)
	if err != nil {
		return nil, err
	}

	return NewClient(APIRootURL, WithRequestEditorFn(basicAuthProvider.Intercept))
}
