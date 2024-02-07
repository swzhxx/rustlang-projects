use nix::{
    sched::{clone, CloneFlags},
    sys::signal::Signal,
    unistd::Pid,
};

use crate::{
    config::ContainerOpts, container::Container, errors::Errcode, hostname::set_container_hostname,
};

const STACK_SIZE: usize = 1024 * 1024;

fn child(config: ContainerOpts) -> isize {
    match setup_container_configurations(&config) {
        Ok(_) => log::info!("Container setup successfully"),
        Err(e) => {
            log::error!("Error while configuring container:{:?}", e);
            return -1;
        }
    }
    log::info!(
        "Starting container with command {} and args {:?}",
        config.path.to_str().unwrap(),
        config.argv
    );
    0
}

pub fn generate_child_process(config: ContainerOpts) -> Result<Pid, Errcode> {
    let mut tmp_stack: [u8; STACK_SIZE] = [0; STACK_SIZE];
    let mut flags = CloneFlags::empty();

    // Flags definition
    match clone(
        Box::new(|| child(config.clone())),
        &mut tmp_stack,
        flags,
        Some(Signal::SIGCHLD as i32),
    ) {
        Ok(pid) => Ok(pid),
        Err(_) => Err(Errcode::ChildProcessError(0)),
    }
}

fn setup_container_configurations(config: &ContainerOpts) -> Result<(), Errcode> {
    set_container_hostname(&config.hostname)?;
    Ok(())
}
