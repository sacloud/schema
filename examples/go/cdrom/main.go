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

	// createCDROM
	created, err := createCDROM(ctx, client)
	if err != nil {
		panic(err)
	}
	fmt.Printf("***** Create *****\n%#v\n\n", created)

	// updateCDROM
	updated, err := updateCDROM(ctx, client, created.ID)
	if err != nil {
		panic(err)
	}
	fmt.Printf("***** Update *****\n%#v\n\n", updated)

	// readCDROM
	read, err := readCDROM(ctx, client, updated.ID)
	if err != nil {
		panic(err)
	}
	fmt.Printf("***** Read *****\n%#v\n\n", read)

	// List
	listed, err := listCDROM(ctx, client)
	if err != nil {
		panic(err)
	}
	fmt.Printf("***** List *****\n%#v\n\n", listed)

	// deleteCDROM
	deleted, err := deleteCDROM(ctx, client, updated.ID)
	if err != nil {
		panic(err)
	}
	fmt.Printf("***** Delete *****\n%#v\n\n", deleted)
}

func createCDROM(ctx context.Context, client *sacloud.Client) (*sacloud.CDROM, error) {
	res, err := client.CreateCDROM(ctx, sacloud.CreateCDROMJSONRequestBody{
		CDROM: sacloud.CDROMCreateRequestBody{
			Name:        "from-openapi-generator",
			SizeMB:      5120,
		},
	})
	if err != nil {
		return nil, err
	}

	createResponse, err := sacloud.ParseCreateCDROMResponse(res)
	if err != nil {
		return nil, err
	}
	return &createResponse.JSON201.CDROM, nil
}

func updateCDROM(ctx context.Context, client *sacloud.Client, id sacloud.ID) (*sacloud.CDROM, error) {
	// updateCDROM
	name := "from-openapi-generator-upd"
	tags := []string{"tag1", "tag2"}

	res, err := client.UpdateCDROMById(ctx, id, sacloud.UpdateCDROMByIdJSONRequestBody{
		CDROM: sacloud.CDROMUpdateRequestBody{
			Name: &name,
			Tags: &tags,
		},
	})
	if err != nil {
		return nil, err
	}

	updateResponse, err := sacloud.ParseUpdateCDROMByIdResponse(res)
	if err != nil {
		return nil, err
	}
	return &updateResponse.JSON200.CDROM, nil
}

func readCDROM(ctx context.Context, client *sacloud.Client, id sacloud.ID) (*sacloud.CDROM, error) {
	res, err := client.ShowCDROMById(ctx, id)
	if err != nil {
		return nil, err
	}

	readResponse, err := sacloud.ParseShowCDROMByIdResponse(res)
	if err != nil {
		return nil, err
	}
	return &readResponse.JSON200.CDROM, err
}

func deleteCDROM(ctx context.Context, client *sacloud.Client, id sacloud.ID) (*sacloud.CDROM, error) {
	res, err := client.DeleteCDROMById(ctx, id)
	if err != nil {
		return nil, err
	}

	deleteResponse, err := sacloud.ParseDeleteCDROMByIdResponse(res)
	if err != nil {
		return nil, err
	}
	return &deleteResponse.JSON200.CDROM, err
}

func listCDROM(ctx context.Context, client *sacloud.Client) (*sacloud.CDROMs, error) {
	s := sacloud.Scope_user
	res, err := client.ListCDROMs(ctx, sacloud.CDROMFindRequest{
		Filter: &sacloud.CDROMFilter{
			Scope: &s,
		},
		AdditionalProperties: nil,
	})
	if err != nil {
		return nil, err
	}

	listResult, err := sacloud.ParseListCDROMsResponse(res)
	if err != nil {
		return nil, err
	}

	return &listResult.JSON200.CDROMs, nil
}