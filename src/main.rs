use std::fs;
use std::path::Path;

fn tree(path: &Path, prefix: &str, show_hidden: bool, max_depth: Option<usize>) {
    if !show_hidden && path.file_name().and_then(|n| n.to_str()).map(|s| s.starts_with('.')).unwrap_or(false) {
        return;
    }

    if max_depth == Some(0) {
        return;
    }

    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        println!("{}{}", prefix, file_name);
    }

    if path.is_dir() {
        let entries = fs::read_dir(path).expect("Failed to read directory entries");

        let num_entries = entries.fold(0, |acc, _entry| acc + 1);
        for (i, entry) in fs::read_dir(path).expect("Failed to read directory entries").enumerate() {
            let entry = entry.expect("Failed to get directory entry");
            let child_path = entry.path();

            let is_last_entry = i == num_entries - 1;
            let new_prefix = format!("{}{} ", prefix, if is_last_entry { "└──" } else { "├──" });
            tree(&child_path, &new_prefix, show_hidden, max_depth.map(|d| d - 1));
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let path = if args.len() > 1 {
        Path::new(&args[1])
    } else {
        Path::new(".")
    };

    let show_hidden = args.iter().any(|arg| arg == "-a" || arg == "--all");
    let max_depth = args.iter().find_map(|arg| {
        if arg.starts_with("-d") || arg.starts_with("--max-depth=") {
            let parts: Vec<&str> = arg.splitn(2, "=").collect();
            if parts.len() == 2 {
                parts[1].parse::<usize>().ok()
            } else {
                None
            }
        } else {
            None
        }
    });

    tree(&path, "", show_hidden, max_depth);
}
