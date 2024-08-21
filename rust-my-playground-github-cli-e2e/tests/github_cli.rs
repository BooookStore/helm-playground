use core::panic;
use std::{
    env, fs,
    process::{Command, Output},
};

use cucumber::{given, when, World};

#[derive(World, Default, Debug)]
struct GithubCliWorld {
    args: Vec<String>,
    output: Option<Output>,
}

impl GithubCliWorld {
    fn add_arg(&mut self, arg: String) {
        self.args.push(arg);
    }
}

#[given(expr = "set environment variable {word} is {word}")]
async fn given_set_environment_variable(_world: &mut GithubCliWorld, key: String, value: String) {
    env::set_var(key, value);
}

#[given(expr = "set arg {word}")]
async fn given_set_arg(world: &mut GithubCliWorld, arg: String) {
    world.add_arg(arg);
}

#[when("run application")]
async fn when_run_application(world: &mut GithubCliWorld) {
    let args = world.args.clone();
    let output = Command::new("./rust-my-playground-github-cli")
        .args(args)
        .output()
        .expect("Failed run application");
    world.output = Some(output);
}

#[tokio::main]
async fn main() {
    build_and_copy();
    GithubCliWorld::cucumber()
        .run_and_exit("tests/features")
        .await;
    clean();
}

fn build_and_copy() {
    let output = Command::new("cargo")
        .arg("build")
        .current_dir("../rust-my-playground-github-cli")
        .output()
        .expect("Error on cargo build");

    if !output.status.success() {
        panic!("Failed on cargo build");
    }

    fs::copy(
        "../rust-my-playground-github-cli/target/debug/rust-my-playground-github-cli",
        "./rust-my-playground-github-cli",
    )
    .expect("Failed copy");
}

fn clean() {
    fs::remove_file("./rust-my-playground-github-cli").expect("Failed remove file");
}
