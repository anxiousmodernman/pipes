use std::process::Command;
use std::process::Stdio;
use libc;
use libc::pid_t;
use std::thread::sleep;
use std::time::Duration;
use std::sync::atomic::{AtomicU32, Ordering};
use lazy_static::lazy_static;
use std::ptr::null;

static  CHILD: AtomicU32 = AtomicU32::new(0);

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
    let mut ptr: i32 = 0;
        match unsafe { libc::waitpid(pid as i32, &mut ptr, 0) } {
            -1 =>  println!{ "errno: {}", errno()},
            pid =>  println!("waitpid: {}", pid)
        }
    std::process::exit(42);
}

// From ion shell: https://gitlab.redox-os.org/redox-os/ion/blob/master/members/sys/src/sys/unix/mod.rs#L40
#[cfg(target_os = "linux")]
fn errno() -> i32 { unsafe { *libc::__errno_location() } }
