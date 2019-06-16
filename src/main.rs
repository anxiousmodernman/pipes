use std::process::Command;

fn main() {
    println!("Hello, world!");

    // spawn the forever writer with regular
    let mut write_forever = Command::new("./write-forever.sh");
    let mut write_child = write_forever.spawn().unwrap();

    write_child.wait();
}
