// use std::process::Command;

#[path = "./deploy/packed.rs"]
mod packed;
use crate::packed::deploy::Deploy;
use crate::packed::Packed;

fn main() {
    println!("Hello, world of rustical deployments!");
    let deploy_packed = Packed{
        project_config: String::from("paulownie")
    };
    deploy_packed.init();
    deploy_packed.deploy();
    deploy_packed.release();
}
