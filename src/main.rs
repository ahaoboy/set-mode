use std::{fs::Permissions, os::unix::fs::PermissionsExt};

pub(crate) fn metadata(file_path: &str) {
    if let Ok(metadata) = std::fs::metadata(file_path) {
        println!("{:#?}", metadata);
        metadata.permissions().mode();
    }
}

fn main() {
    let (file, cmd, output) = (
        std::env::args().nth(1),
        std::env::args().nth(2).and_then(|i| i.parse::<u32>().ok()),
        std::env::args().nth(3),
    );
    match (file, cmd, output) {
        (Some(file), None, _) => metadata(&file),
        (Some(file), Some(mode), None) => {
            let p = Permissions::from_mode(mode);
            std::fs::set_permissions(&file, p).expect("set set_permissions error");
            metadata(&file);
        }
        (Some(file), Some(mode), Some(output)) => {
            let p = Permissions::from_mode(mode);
            let b = std::fs::read(&file).expect("read file error");
            std::fs::write(&output, b).expect("write file error");
            std::fs::set_permissions(&output, p).expect("set set_permissions error");
            metadata(&output);
        }
        _ => println!("set-mode file mode"),
    }
}
