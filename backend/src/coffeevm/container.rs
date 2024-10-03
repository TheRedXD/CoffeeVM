use std::sync::RwLock;

use bollard::{container::{AttachContainerOptions, AttachContainerResults}, errors::Error, Docker};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ContainerState {
    Offline,
    Online,
    Starting,
    Stopping
}

pub struct Container<'a> {
    pub tag: String,
    pub name: String,
    pub docker: &'a Docker,
    pub state: RwLock<ContainerState>
}
impl<'a> Container<'a> {
    pub fn full_name(&self) -> String {
        let mut name = self.tag.to_owned();
        name.push_str(&self.name);
        name
    }
    pub fn get_state(&self) -> ContainerState {
        let rl = self.state.read().unwrap();
        *rl
    }
    pub async fn start(&self) -> Result<(), Error> {
        let mut w = self.state.write().unwrap();
        *w = ContainerState::Starting;
        let val = self.docker.start_container(&self.full_name(), Some(bollard::container::StartContainerOptions { detach_keys: "ctrl-^" })).await;
        *w = ContainerState::Online;
        val
    }
    pub async fn stop(&self) -> Result<(), Error> {
        let mut w = self.state.write().unwrap();
        *w = ContainerState::Stopping;
        let val = self.docker.stop_container(&self.full_name(), Some(bollard::container::StopContainerOptions {
            t: 0
        })).await;
        *w = ContainerState::Offline;
        val
    }
    pub async fn kill(&self) -> Result<(), Error> {
        let mut w = self.state.write().unwrap();
        *w = ContainerState::Stopping;
        let val = self.docker.kill_container(&self.full_name(), Some(bollard::container::KillContainerOptions {
            signal: "SIGINT"
        })).await;
        *w = ContainerState::Offline;
        val
    }
    pub async fn attach(&self) -> Result<AttachContainerResults, Error> {
        self.docker.attach_container(&self.full_name(), Some(AttachContainerOptions {
            logs: Some(false),
            stderr: Some(false),
            stdin: Some(true),
            stdout: Some(false),
            stream: Some(false),
            detach_keys: Some("ctrl-^")
        })).await
    }
    pub fn set_state(&self, state: ContainerState) {
        let mut w = self.state.write().unwrap();
        *w = state;
    }
}