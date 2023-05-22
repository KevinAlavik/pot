use std::path::{Path, PathBuf};
use std::fs;

fn main() {
    let input_file = Path::new("target").join(env!("PROFILE")).join(env!("CARGO_BIN_NAME"));
    let output_file = Path::new("/usr/local/bin").join(env!("CARGO_BIN_NAME"));

    // Create the output directory if it doesn't exist
    fs::create_dir_all("/usr/local/bin").expect("Failed to create output directory");

    // Copy the executable to the destination path
    fs::copy(&input_file, &output_file).expect("Failed to copy the executable");

    // Set executable permissions for the output file
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(&output_file)
            .expect("Failed to get metadata for output file")
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&output_file, permissions).expect("Failed to set permissions for output file");
    }
}
