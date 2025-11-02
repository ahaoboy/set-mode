use std::{fs::Permissions, os::unix::fs::PermissionsExt};

pub(crate) fn metadata(file_path: &str) {

    if let Ok(metadata) = std::fs::metadata(file_path) {
        println!("{:#?}", metadata);
    }
}

fn main() {
    let (file, cmd) = (
        std::env::args().nth(1),
        std::env::args().nth(2).and_then(|i| i.parse::<u32>().ok()),
    );
    match (file, cmd) {
        (Some(file), None) => metadata(&file),
        (Some(file), Some(mode)) => {
            let p = Permissions::from_mode(mode);
            if std::fs::set_permissions(&file, p).is_ok() {
                metadata(&file)
            }
        }
        _ => println!("set-mode file mode"),
    }
}
