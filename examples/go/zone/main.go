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

	res, err := client.ListZones(ctx, sacloud.ZoneFindRequest{})
	if err != nil {
		panic(err)
	}

	listResult, err := sacloud.ParseListZonesResponse(res)
	if err != nil {
		panic(err)
	}

	fmt.Printf("Zones: %#v\n", listResult.JSON200)
}
