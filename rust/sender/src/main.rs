#![feature(stdin_forwarders)]
use nix::unistd::getpid;
use std::{thread, time::Duration};
use std::io;
use std::io::Read;
use nix::sys::signal::{kill, signal, SigHandler, Signal};
use nix::sys::signal;
use nix::unistd::Pid;
use std::convert::TryFrom;
use libc;
use libc::{siginfo_t, c_int, c_void};
use helper::find_pid;

extern "C" fn handle_signal(signum: libc::c_int) {
    if Signal::try_from(signum).unwrap() == Signal::SIGTSTP {
        println!("SIGTSTP recieved");
    }
}

extern "C" fn sighandler(_: c_int, info: *mut siginfo_t, _: *mut c_void) {
    // TODO 5-3: upon receiving SIGTSTP, print "SIGTSTP received"
}

fn main() {
    let pid: i32 = find_pid("./handler".to_string());
    // TODO 5-2: similar to TODO 1-1, register signal handler for SIGTSTP

    if pid > 0 {
        println!("Send SIGINT to handler (pid: {})\n", pid);
        // TODO 4: send SIGINT signal to pid using "kill" function
    }
    thread::sleep(Duration::from_secs(10));
    return;
}