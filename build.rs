use std::process::Command;

fn main() {
    let mut cmd = Command::new("rustc");
    cmd.arg("+nightly")
        .arg("src/main.rs")
        .arg("-Ctarget-cpu=native")
        .arg("-Clink-args=-nostartfiles")
        .arg("-Clink-args=-Wl,-n,-N,--no-dynamic-linker,--no-pie,--build-id=none")
        .arg("-o")
        .arg("hello_world");

    let output = cmd.output().expect("failed to execute process");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("rustc failed: {}", stderr);
    }

    println!("Successfully built hello_world");
}
