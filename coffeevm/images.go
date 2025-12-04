package coffeevm

import (
	"context"
	"slices"

	moby "github.com/moby/moby/client"
)

func ExistsImage(ctx context.Context, client *moby.Client, imageName string) (bool, error) {
	images, err := client.ImageList(ctx, moby.ImageListOptions{All: true})
	if err != nil {
		return false, err
	}
	foundImage := false
	for _, image := range images.Items {
		if slices.Contains(image.RepoTags, imageName) {
			foundImage = true
		}
		if foundImage {
			break
		}
	}
	return foundImage, nil
}

func EnsureImage(logchan chan string, ctx context.Context, client *moby.Client, imageName string) (err error) {
	exists, err := ExistsImage(ctx, client, imageName)
	if !exists {
		logchan<-"Image "+imageName+" not found, installing..."
		resp, err := client.ImagePull(ctx, imageName, moby.ImagePullOptions{})
		err = resp.Wait(ctx)
		
		resp.Close()
		return err
	}
	return err
}