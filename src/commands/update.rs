use std::process::Command;

pub fn cli_update() {
    println!("Updating....");
    let _output = Command::new("bash")
        .arg("-c")
        .arg("curl -sSL https://tronlang.org/install.sh | bash")
        .output()
        .expect("Failed to execute command");
    println!("Update completed");
}
