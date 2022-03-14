use std::os::unix::fs::MetadataExt;
use nix::unistd;
use std::fs;

pub fn find_pid(target: String) -> i32 {
    let uid: u32 = u32::from(unistd::getuid());
    for entry in fs::read_dir("/proc").unwrap() {
        let entry = entry.unwrap();
        let pid_dir = entry.file_name().to_str().unwrap().to_string();
        let parsed = pid_dir.parse::<i32>();
        match parsed {
            Err(_) => { 
                continue; // Not a valid number
            }
            Ok(pid) => {
                // TODO 2: under each process dir, there is a file that
                // can help us find the program name. First, read /proc's man page and 
                // then create an absolute path (based on "/proc/%s/[file]" template) 
                // and replace {path} below with it:
                let cmdline = fs::read_to_string({path}).unwrap();
                let cmdline = cmdline.trim_matches(char::from(0));
                if !cmdline.eq(&target) {
                    continue;
                }

                // TODO 3-1: retrieve metadata struct for the file (i.e. path).
                // use fs::metadata. https://doc.rust-lang.org/stable/std/fs/fn.metadata.html
                let metadata = // TODO3: <fill code here>;

                let uid = metadata.uid();
                // TODO 3-2: this if clause checks whether the content of the special file
                // matches the program name we are looking for. Since many students/users  
                // could be running the same program, add additional condition to find the 
                // program that was run under your account (i.e., compare its uid with getuid() value in line 14)
                return pid;
            }
        }
    }
    return 0;
}
