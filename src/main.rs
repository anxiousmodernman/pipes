use std::process::Command;
use std::process::Stdio;
use libc;
use libc::pid_t;
use std::thread::sleep;
use std::time::Duration;
use std::sync::atomic::{AtomicU32, Ordering};
use lazy_static::lazy_static;
use std::ptr::null;

lazy_static! {
    static ref CHILD: AtomicU32 = AtomicU32::new(0);
}

fn main() {
    let mut writer = Command::new("./write-forever.sh")
        .stdout(Stdio::piped()).spawn().unwrap();

    let mut reader = Command::new("./read_forever")
        .stdin(writer.stdout.expect("writer stdout error")).spawn().unwrap();

    // put child pid in global
    let pid = reader.id();
    println!("child has pid: {}", pid);
    CHILD.store(pid, Ordering::Relaxed);

    unsafe { libc::signal(libc::SIGINT, handle_sigint as libc::sighandler_t) };

    loop { sleep(Duration::from_millis(100)) }
}

fn handle_sigint(sig: usize) {
    println!("\nhandlin' it! {}", sig);
    let pid = CHILD.load(Ordering::Relaxed);
    println!("awaiting child pid: {}", pid as i32);
    let mut ptr: *const i32 = null();
    unsafe {
        let res: pid_t = libc::waitpid(pid as i32, ptr as *mut i32, 0);
    }
    std::process::exit(42);
}