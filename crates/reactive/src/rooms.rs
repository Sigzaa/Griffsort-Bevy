use std::process::Command;

pub fn init() {
    println!("initing new room");
    let output = if cfg!(target_os = "windows") {
        Command::new("make")
            .args(["s"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("make")
            .arg("s")
            .output()
            .expect("failed to execute process")
    };
    let hello = output.stdout;
    println!("output: {hello:?}");
}
