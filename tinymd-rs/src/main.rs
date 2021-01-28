fn parse_markdown_file(filename: &str) {
    print_short_banner();
    println!("[ INFO ] Trying to parse {}...", filename);
}

fn get_title() -> String {
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(" (v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    title.push_str("), ");
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    title
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
    println!("Written by: {}\nHomepage: {}\nUsage: tinymd <somefile>.md\n",
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_HOMEPAGE")
    );
}

fn usage() {
    print_long_banner();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => {
            println!("[ ERROR ] You forgot to specify the markdown file to parse!");
            usage();
        }
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("[ ERROR ] Confusing invocation. See usage below.")
            usage();
        }
    }
}
