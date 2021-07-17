use tokio::process::Command;

#[tokio::main]
async fn main() {
    Command::new("ls").status().await.expect("ls command failed to run");
    let output: std::process::Output = Command::new("ls").output().await.expect("ls command failed to run");
    println!("stderr of ls: {:?}", output.stdout);
}
