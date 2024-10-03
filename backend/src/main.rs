#![feature(async_closure)]

use std::future::Future;
use std::io::Error;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use bollard::container::Config;
use bollard::secret::HostConfig;
use bollard::Docker;
use coffeevm::container::ContainerState;
use coffeevm::host::Host;
use tokio::io::AsyncWriteExt;

mod coffeevm;

#[tokio::main]
async fn main() {
    let tag = "coffeevm-";
    #[cfg(unix)]
    let docker = Docker::connect_with_socket_defaults().unwrap();
    let mut host = coffeevm::host::Host::default();
    host.set_tag(tag.into());

    let images = host.list_images(&docker).await.unwrap();

    for image in images {
        println!("-> {:?}", image.repo_tags[0]);
    }

    // let (stop_sender, stop_receiver) = crossbeam_channel::unbounded();

    // ctrlc::set_handler(move || {
    //     stop_sender.send(()).unwrap()
    // }).expect("Unable to create CTRL-C handler!");

    host.create_container(&docker, "test".into(), Config {
        image: Some("ubuntu:latest".into()),
        cmd: Some(vec!["/bin/bash".into()]),
        tty: Some(true),
        open_stdin: Some(true),
        host_config: Some(HostConfig {
            auto_remove: Some(true),
            ..Default::default()
        }),
        ..Default::default()
    }).await.unwrap();

    let container = host.get_container("test".into()).unwrap();

    container.start().await.unwrap();

    let mut att = container.attach().await.unwrap();
    att.input.write("apt update\n".as_bytes()).await.unwrap();
}
