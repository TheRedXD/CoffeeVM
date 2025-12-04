package coffeevm

import (
	"context"

	"github.com/moby/moby/api/types/container"
	"github.com/moby/moby/api/types/network"
	
	moby "github.com/moby/moby/client"
	ocispec "github.com/opencontainers/image-spec/specs-go/v1"
)

type ContainerCreateOptions struct {
	Config *container.Config
	HostConfig *container.HostConfig
	NetworkingConfig *network.NetworkingConfig
	Platform *ocispec.Platform
	Name *string
	Image *string
}

func CreateContainer(logchan chan string, ctx context.Context, client *moby.Client, name string, imageName string, pullImage bool, options ContainerCreateOptions) (string, error) {
	if pullImage {
		err := EnsureImage(logchan, ctx, client, imageName)
		if err != nil {
			return "", err
		}
	}
	fullOptions := moby.ContainerCreateOptions{
		Name: "coffeevm-"+name,
		Image: imageName,
	}
	if options.Config != nil {
		fullOptions.Config = options.Config
	}
	if options.HostConfig != nil {
		fullOptions.HostConfig = options.HostConfig
	}
	if options.NetworkingConfig != nil {
		fullOptions.NetworkingConfig = options.NetworkingConfig
	}
	if options.Platform != nil {
		fullOptions.Platform = options.Platform
	}
	if options.Name != nil {
		fullOptions.Name = *options.Name
	}
	if options.Image != nil {
		fullOptions.Image = *options.Image
	}
	resp, err := client.ContainerCreate(ctx, fullOptions)
	if err != nil {
		return "", err
	}
	
	return resp.ID, nil
}

func DeleteContainer() {
	
}