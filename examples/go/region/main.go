package main

import (
	"context"
	"fmt"

	"github.com/sacloud/schema/examples/go/sacloud"
)

func main() {
	ctx := context.Background()
	client, err := sacloud.NewClientFromEnv()
	if err != nil {
		panic(err)
	}

	res, err := client.ListRegions(ctx, sacloud.RegionFindRequest{})
	if err != nil {
		panic(err)
	}

	listResult, err := sacloud.ParseListRegionsResponse(res)
	if err != nil {
		panic(err)
	}

	fmt.Printf("Regions: %#v\n", listResult.JSON200)
}
