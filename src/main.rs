extern crate getopts;

const INTERFACE_OPT: &'static str = "interface";

#[inline(always)]
fn options() -> getopts::Options {
    let mut opts = getopts::Options::new();
    opts.optmulti("i", INTERFACE_OPT, "interface to check", "iface");
    opts
}

#[inline(never)]
fn real_main() -> i32 {
    use std::env;
    use std::sync;
    use std::thread;

    let opts = options();
    let matches = opts.parse(env::args().skip(1)).unwrap();
    let interfaces = matches.opt_strs(INTERFACE_OPT);
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
