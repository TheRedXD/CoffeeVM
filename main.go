package main

import (
	"context"
	"fmt"
	"os"

	moby "github.com/moby/moby/client"

	"main/coffeevm"
)

func main() {
	logchan := make(chan string)
	
	go func() {
		for {
			fmt.Println(<-logchan)
		}
	}()
	
	client, err := moby.New(moby.WithAPIVersion("1.52"));
	if err != nil {
		println(err.Error())
		os.Exit(1)
		return
	}
	ctx := context.Background()
	
	err = coffeevm.EnsureImage(logchan, ctx, client, "alpine:latest")
	if err != nil {
		println(err.Error())
		os.Exit(1)
		return
	}

	resp, err := client.ContainerCreate(ctx, moby.ContainerCreateOptions{
		Image: "alpine:latest",
	})
	if err != nil {
		println(err.Error())
		os.Exit(1)
		return
	}
	println("Container created:", resp.ID)
	_, err = client.ContainerRemove(ctx, resp.ID, moby.ContainerRemoveOptions{Force: true, RemoveVolumes: true, RemoveLinks: false})
	if err != nil {
		println(err.Error())
		os.Exit(1)
		return
	}
	_ = client.Close()
}