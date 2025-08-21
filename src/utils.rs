use std::path::Path;

pub fn copy_dir<P: AsRef<Path>>(from: P, to: P) -> std::io::Result<()> {
    let entries = std::fs::read_dir(from)?;

    std::fs::create_dir_all(to.as_ref())?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        let dest = to.as_ref().to_path_buf().join(path.file_name().unwrap());

        if path.is_dir() {
            copy_dir(&path, &dest)?;
        } else {
            std::fs::copy(&path, &dest)?;
        }
    }

    Ok(())
}

pub fn slugify(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}
