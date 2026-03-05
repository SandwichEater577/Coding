use std::env;
use std::process::Command;
use std::path::Path;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Jeśli użyjesz aliasu, Twoim plikiem będzie args[1] lub args[2]
    // Sprawdzamy, który argument nie jest flagą i nie jest nazwą programu
    let file_to_launch = args.iter()
        .skip(1)
        .find(|&a| a != "--launch")
        .map(|s| s.as_str());

    match file_to_launch {
        Some(file) => handle_launch(file),
        None => print_box(" ERROR ", "Usage: --launch <filename>"),
    }
}

fn handle_launch(file: &str) {
    let path = Path::new(file);

    if !path.exists() {
        print_box(" ERROR ", &format!("File '{}' not found!", file));
        return;
    }

    match path.extension().and_then(|s| s.to_str()) {
        Some("c") => {
            print_box(" COMPILING C ", file);
            if run_cmd("gcc", &[file, "-o", "temp_out"]) {
                run_cmd("./temp_out", &[]);
                let _ = fs::remove_file("temp_out"); // Sprzątanie po sobie
            }
        }
        Some("rs") => {
            print_box(" CARGO RUN ", file);
            run_cmd("cargo", &["run"]);
        }
        Some("py") => {
            print_box(" PYTHON 3 ", file);
            run_cmd("python3", &[file]);
        }
        _ => print_box(" ERROR ", "Unsupported file extension."),
    }
}

fn print_box(title: &str, msg: &str) {
    let line = "=".repeat(msg.len() + 4);
    println!("\n  .{}.", line);
    println!("  | [{}] {}", title, msg);
    println!("  '{}'\n", line);
}

fn run_cmd(cmd: &str, args: &[&str]) -> bool {
    Command::new(cmd)
        .args(args)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}