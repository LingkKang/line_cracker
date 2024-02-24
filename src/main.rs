const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const MAX_LEN: usize = 80;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();

    for arg in args.iter() {
        println!("[DEBUG] {}", arg);
    }

    if args.get(1) == Some(&PKG_NAME.to_string()) {
        println!("[DEBUG] Called inside `cargo`, removing the first arg");
        args.remove(1);
    }

    let len = args.len();
    println!("[DEBUG] len = {}", len);

    if len != 2 {
        print_version();
        print_help();
        std::process::exit(0);
    }

    let first_arg = args.pop().unwrap();

    if (first_arg == "-h") || (first_arg == "--help") {
        print_help();
        std::process::exit(0);
    }

    if (first_arg == "-v") || (first_arg == "--version") {
        print_version();
        std::process::exit(0);
    }

    check_file(&first_arg);

    println!("[INFO] checked: {} ", first_arg);
}

fn print_help() {
    println!("Usage: {} <filename>", PKG_NAME);
}

fn print_version() {
    println!("{} {} by {}", PKG_NAME, VERSION, env!("CARGO_PKG_AUTHORS"));
}

fn check_file(file_path: &str) {
    let path = std::path::Path::new(file_path);
    if !path.exists() {
        eprintln!("{}: No such file or directory", file_path);
        std::process::exit(1);
    }
    if !path.is_file() {
        eprintln!("{}: Is not a file", file_path);
        std::process::exit(1);
    }

    if std::fs::read_to_string(path).unwrap().contains('\0') {
        println!("[INFO] file corrupted or non-text, skipped: {}", file_path);
        std::process::exit(0);
    }

    for (line_num, line) in std::fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .enumerate()
    {
        let length = line.len();
        if length > MAX_LEN {
            println!(
                "[WARN] {}:{}: expected {} chars but got {}",
                file_path,
                line_num + 1,
                MAX_LEN,
                length
            );
        }
    }
}
