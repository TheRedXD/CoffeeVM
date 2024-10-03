use std::sync::RwLock;
use futures::stream::StreamExt;

use bollard::{container::{Config, CreateContainerOptions}, errors::Error, image::ListImagesOptions, secret::ImageSummary, Docker};

use crate::coffeevm::container::{Container, ContainerState};

#[derive(Default)]
pub struct Host<'a> {
    pub tag: String,
    pub containers: std::collections::HashMap<String, Container<'a>>
}

impl<'a> Host<'a> {
    pub fn set_tag(&mut self, string: String) {
        self.tag = string;
    }
    pub fn add_container(&mut self, name: String, container: Container<'a>) {
        self.containers.insert(name, container);
    }
    pub fn remove_container(&mut self, name: String) {
        self.containers.remove(&name);
    }
    pub fn get_container(&mut self, name: String) -> Option<&Container> {
        self.containers.get(&name)
    }
    pub async fn create_container(&mut self, docker: &'a Docker, name: String, options: Config<String>) -> Option<&Container> {
        docker.create_container(Some(CreateContainerOptions {
            name: self.tag.to_owned() + name.as_str(),
            platform: None
        }), options).await.unwrap();
        self.containers.insert(name.clone(), Container {
            tag: self.tag.clone(),
            name: name.clone(),
            docker: &docker,
            state: RwLock::new(ContainerState::Offline)
        });
        self.containers.get(&name)
    }
    pub async fn list_images(&self, docker: &Docker) -> Result<Vec<ImageSummary>, Error> {
        docker.list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        })).await
    }
    pub async fn stop_all_containers(&self) {
        futures::stream::iter(&self.containers).for_each_concurrent(None, |(_, container)| async {
            container.stop().await.unwrap()
        }).await
    }
    pub async fn kill_all_containers(&self) {
        futures::stream::iter(&self.containers).for_each_concurrent(None, |(_, container)| async {
            container.kill().await.unwrap()
        }).await
    }
}