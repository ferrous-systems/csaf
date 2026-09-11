use clap::Parser;

fn main() {
    let cmd = csaf_rs::Cmd::parse();

    csaf_rs::run(cmd);
}
