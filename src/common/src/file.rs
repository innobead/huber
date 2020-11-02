use std::path::Path;

pub fn is_empty_dir<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref()
        .read_dir()
        .map(|mut it| it.next().is_none())
        .unwrap_or(false)
}
