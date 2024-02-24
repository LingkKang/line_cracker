/// As the binary file at testing time will be added with a file hash,
/// we need to find the binary file first.
fn find_binary() -> Option<std::path::PathBuf> {
    let pkg_name = env!("CARGO_CRATE_NAME");
    let target_dir = "./target/debug/deps/";
    let entries =
        std::fs::read_dir(std::path::PathBuf::from(target_dir)).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file()
            && path.file_stem()?.to_str()?.starts_with(pkg_name)
            && path.extension()?.to_str()? == "exe"
        {
            return Some(path);
        }
    }

    None
}

#[test]
fn test_binary() {
    let bin_path = find_binary();
    assert!(bin_path.is_some(), "Binary not found");

    let bin_path = bin_path.unwrap();

    let output = std::process::Command::new(bin_path)
        .arg("./tests/data/lorem_ipsum.txt")
        .output()
        .expect("Failed to execute process");

    assert!(output.status.success(), "Command failed");

    // println!("...");

    // let output_str = std::str::from_utf8(&output.stdout).unwrap();

    // std::fs::write("./temp.txt", output_str).unwrap();

    // for (i, line) in output_str.lines().enumerate() {
    //     // match i {
    //     //     0 => assert_eq!(line, "[INFO] checked: ./tests/data/lorem_ipsum.txt"),
    //     //     _ => panic!("Unexpected output"),
    //     // }
    //     println!("{}: {}", i, line);
    // }

    // assert!(true);

    // for (i, line) in output.stdout.
    // lines().enumerate() {
    //     println!("{}: {}", i, line.unwrap());
    // }

    // let output_str = std::str::from_utf8(&output.stdout).unwrap();

    // assert_eq!(output.status.success(), true);
    // assert_eq!(
    //     String::from_utf8_lossy(&output.stdout),
    //     "[INFO] checked: tests/cli.rs \n"
    // );
}
