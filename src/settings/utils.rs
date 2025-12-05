use std::path::PathBuf;

pub fn build_path(base: &PathBuf, master: &str, sub: &str) -> PathBuf {
    let mut p = base.clone();

    if master == "master" {
        p.push(sub);
    } else {
        p.push(master);
        p.push(sub);
    }

    p
}