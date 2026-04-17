use std::fs;
use std::path::Path;
use console::style;

fn tree(path: &Path, depth: usize) {
  let indent = "  ".repeat(depth);
  println!(
    "{}{}",
    indent,
    path.file_name().unwrap_or_default().to_string_lossy()
  );
  let Ok(entries) = fs::read_dir(path) else {
    return;
  };
  for entry in entries.flatten() {
    let path = entry.path();
    if path.is_dir() {
      tree(&path, depth + 1)
    } else {
      println!(
        "{}  {}",
        indent,
        path.file_name().unwrap_or_default().to_string_lossy()
      )
    }
  }
}

fn request_full_disk_access() {
	open::that("x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles").unwrap();
	println!("{}", style("⚠️ Full Disk Access required").yellow().bold());
}

fn has_full_disk_access() -> bool {
	std::fs::read_dir("/Library/Application Support/com.apple.TCC").is_ok()
}

fn main() {
	if !has_full_disk_access() {
		request_full_disk_access();
		return;
	}

  tree(Path::new("/"), 0);
}
