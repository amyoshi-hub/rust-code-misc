use std::process::Command;

fn main() {
    let output = Command::new("aplay")
        .arg("output.wav")
        .output()
        .expect("faild to call");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
