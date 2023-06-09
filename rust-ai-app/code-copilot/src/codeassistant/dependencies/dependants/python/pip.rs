use std::process::Output;
use tokio::process::{Command};

pub async fn run_pip_install(module_name: &str) -> Result<Output, tokio::io::Error> {
    println!("Pip Command: pip install {}",&module_name);
    Command::new("pip")
       .arg("install")
       .arg(module_name)
       .output()
       .await
}

pub async fn check_pip_installed() -> bool {
    let output = Command::new("pip").arg("--version").output().await;

    match output {
        Ok(_) => true,  // pip is installed
        Err(_) => false,  // pip is not installed
    }
}

pub async fn install_python_and_pip() -> Result<std::process::Output, tokio::io::Error> {
    Command::new("apt")
        .arg("install")
        .arg("-y")
        .arg("python3") // Update with the appropriate package name for Python 3 on your Linux distribution
        .output()
        .await
}