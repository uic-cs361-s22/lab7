#![feature(stdin_forwarders)]
use nix::unistd::{getpid, Pid};
use signal_hook::{iterator::SignalsInfo};
use signal_hook::consts::signal::{SIGINT, SIGTSTP};
use std::{thread, time::Duration};
use signal_hook::iterator::exfiltrator::origin::WithOrigin;
use std::io;
use std::io::Read;
use nix::sys::signal::{kill, signal, SigHandler, Signal};
use std::convert::TryFrom;
use libc;
use helper::find_pid;

fn send_signal() {
    // TODO 5-1: send SIGTSTP signal to ./sender program
    // 1. find ./sender program's pid
    // 2. send SIGTSTP signal to its pid 	
    // 3. call this function at the bottom of signal_handler 
}

// signal handler
extern "C" fn handle_signal(signum: libc::c_int) {
        match Signal::try_from(signum).unwrap() {
            // TODO 1-2: define a behavior to react when SIGTSTP (ctrl+Z) or SIGINT (ctrl+C) is received
            // For instance, print a message that "SIGTSTP received" or "SIGINT received"

            // TODO 5-1 (continue): call this function if the received signal is SIGINT
            // send_signal(SIGTSTP);
        }
        println!("Type exit to quit the program...");
}

fn main() {

    // TODO 1-1: register signal handler for SIGTSTP and SIGINT

    println!("Type exit to quit the program...");
    let lines = io::stdin().lines();
    for line in lines {
        // wait for exit
        if line.unwrap().eq("exit") {
            break;
        }
    }
    
    return;
}