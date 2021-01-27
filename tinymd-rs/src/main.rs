fn get_version() -> u16 {
    1000
}

fn usage() {
    let version = get_version();
    println!("tinymd, a markdown compiler written by Kashif");
    println!("Version {}", version);
}

fn main() {
    usage();
}
