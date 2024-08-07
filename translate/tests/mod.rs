use std::process::Command;

use testcontainers::{
    core::WaitFor, runners::AsyncRunner, ContainerRequest, GenericImage, ImageExt,
};

const IMAGE_NAME: &str = "test-container";

pub async fn run_docker_image(container_cmd: String) {
    GenericImage::new("rust", "1.80-alpine")
        .with_exposed_port(testcontainers::core::ContainerPort::Tcp(3000))
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
        .with_name(IMAGE_NAME)
        .with_cmd(container_cmd.split_whitespace())
        .start()
        .await
        .unwrap()
        .start()
        .await
        .unwrap();
}

pub fn copy_compiled_binary_to_docker() {
    let container_cmd =
        "cargo build && ./target/debug/translate --from zh-cn --engine google --to en --text '狗' ";
}
