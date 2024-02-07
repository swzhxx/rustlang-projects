use nix::unistd::sethostname;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::errors::Errcode;

#[rustfmt::skip]
const HOSTNAME_NAMES: [&'static str; 8] = [
    "cat", "world", "coffee", "girl",
    "man", "book", "pinguin", "moon"];
#[rustfmt::skip]
const HOSTNAME_ADJ: [&'static str; 16] = [
    "blue", "red", "green", "yellow",
    "big", "small", "tall", "thin",
    "round", "square", "triangular", "weird",
    "noisy", "silent", "soft", "irregular"];

pub fn generate_hostname() -> Result<String, Errcode> {
    let mut rng = rand::thread_rng();
    let num = rng.gen::<u8>();
    let name = HOSTNAME_NAMES.choose(&mut rng).ok_or(Errcode::RngError)?;
    let adj = HOSTNAME_ADJ.choose(&mut rng).ok_or(Errcode::RngError)?;
    Ok(format!("{}-{}-{}", adj, name, num))
}

pub fn set_container_hostname(hostname: &String) -> Result<(), Errcode> {
    match sethostname(hostname) {
        Ok(_) => {
            log::debug!("Container hostname is now {}", hostname);
            Ok(())
        }
        Err(_) => {
            log::error!("Cannot set hostname {} for container", hostname);
            Err(Errcode::HostnameError(0))
        }
    }
}
