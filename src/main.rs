use std::process::Command;
use tokio_process::CommandExt; // extension trait for std::process::Command;
use std::process::Stdio;
use libc;
use std::thread::sleep;
use std::time::Duration;
use std::sync::atomic::{AtomicU32, Ordering};

static CHILD: AtomicU32 = AtomicU32::new(0);

fn main() {

    // foreground_std_lib();
    foreground_tokio();

    loop { sleep(Duration::from_millis(100)) }
}

fn foreground_std_lib() {
    let writer = Command::new("./write-forever.sh")
        .stdout(Stdio::piped()).spawn().unwrap();

    let reader = Command::new("./read_forever")
        .stdin(writer.stdout.expect("writer stdout error")).spawn().unwrap();

    // put child pid in global
    let pid = reader.id();
    println!("child has pid: {}", pid);
    CHILD.store(pid, Ordering::Relaxed);

    unsafe { libc::signal(libc::SIGINT, handle_sigint as libc::sighandler_t) };
}

fn foreground_tokio() {
    let mut writer = Command::new("./write-forever.sh");
    writer.stdout(Stdio::piped());
    
    let mut reader = Command::new("./read_forever");
    reader.stdin(Stdio::piped());
    let mut wr = writer.spawn_async().unwrap().stdout().take().unwrap();
    let mut rd = reader.spawn_async().unwrap().stdin().take().unwrap();

    use std::io;
    // What's the Tokio equivalent of this?
    // io::copy(&mut wr, &mut rd);
}

fn handle_sigint(sig: usize) {
    println!("\nhandlin' it! {}", sig);
    let pid = CHILD.load(Ordering::Relaxed);
    println!("awaiting child pid: {}", pid as i32);
    let mut ptr: i32 = 0;
    match unsafe { libc::waitpid(pid as i32, &mut ptr, 0) } {
        -1 => println! {"errno: {}", errno()},
        pid => println!("waitpid: {}", pid)
    }
    std::process::exit(42);
}


// From ion shell: https://gitlab.redox-os.org/redox-os/ion/blob/master/members/sys/src/sys/unix/mod.rs#L40
#[cfg(target_os = "linux")]
fn errno() -> i32 { unsafe { *libc::__errno_location() } }
