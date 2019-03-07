extern crate getopts;

use std::io;

const INTERFACE_OPT: &'static str = "interface";
const IGNORE_OPT: &'static str = "ignore";

fn all_interfaces() -> io::Result<Vec<String>> {
    use std::fs;
    fs::read_dir("/sys/class/net")
        .and_then(|entries| {
            entries.fold(Ok(Vec::with_capacity(5)), |acc, entry_res| {
                acc
                    .and_then(move |list: Vec<String>| entry_res.map(move |entry| (list, entry)))
                    .map(|(mut list, entry)| {
                        list.push(entry.file_name().into_string().unwrap());
                        list
                    })
            })
        })
}

#[inline(always)]
fn options() -> getopts::Options {
    let mut opts = getopts::Options::new();
    opts.optmulti("i", INTERFACE_OPT, "interface to check", "iface");
    opts.optmulti("", IGNORE_OPT, "interface to ignore", "iface");
    opts
}

#[inline(never)]
fn real_main() -> i32 {
    use std::env;
    use std::sync;
    use std::thread;

    let opts = options();
    let matches = opts.parse(env::args().skip(1)).unwrap();
    let mut interfaces = matches.opt_strs(INTERFACE_OPT);
    if interfaces.is_empty() {
        interfaces = all_interfaces().unwrap();
    }
    let mut ignore_interfaces = matches.opt_strs(IGNORE_OPT);
    if ignore_interfaces.iter().find(|&s| s.eq("lo")).is_none() {
        ignore_interfaces.push("lo".to_string());
    }
    let (tx, rx) = sync::mpsc::channel();
    for iface in interfaces {
        let tx = tx.clone();
        thread::spawn(move || {
            use std::process::Command;
            let status = Command::new("/usr/lib/systemd/systemd-networkd-wait-online")
                .arg("-i").arg(&iface)
                .status().ok()
                .filter(std::process::ExitStatus::success)
                .map(move |_| iface);
            tx.send(status).unwrap();
        });
    }
    let mut success = 1;
    while let Ok(status) = rx.recv() {
        if status.is_some() {
            success = 0;
            break;
        }
    }
    success
}

fn main() {
    let exit_code = real_main();
    std::process::exit(exit_code);
}
