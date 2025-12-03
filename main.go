package main

import (
	"context"
	"io"
	"os"
	"slices"

	moby "github.com/moby/moby/client"
)

func main() {
	client, err := moby.New(moby.WithAPIVersion("1.52"));
	if err != nil {
		println(err.Error())
		os.Exit(1)
		return
	}
	ctx := context.Background()
	images, _ := client.ImageList(ctx, moby.ImageListOptions{All: true})
	foundImage := false
	for _, image := range images.Items {
		if slices.Contains(image.RepoTags, "alpine:latest") {
			foundImage = true
		}
		if foundImage {
			break
		}
	}
	if !foundImage {
		println("Image alpine:latest not found, installing")
		resp, err := client.ImagePull(ctx, "alpine:latest", moby.ImagePullOptions{})
		if err != nil {
			println(err.Error())
			os.Exit(1)
			return
		}
		defer resp.Close()
		_, err = io.Copy(io.Discard, resp)
		if err != nil {
			println(err.Error())
			os.Exit(1)
			return
		}
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