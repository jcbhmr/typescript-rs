#[cfg(test)]
mod tests {
    const TYPESCRIPT_VERSION: &str = "5.5.2";

    #[test]
    fn typescript_tgz() -> Result<(), Box<dyn std::error::Error>> {
        let sh = xshell::Shell::new()?;
        eprintln!("TYPESCRIPT_VERSION={:?}", TYPESCRIPT_VERSION);
        let output = xshell::cmd!(sh, "npm pack typescript@{TYPESCRIPT_VERSION}").output()?;
        let out = std::str::from_utf8(&output.stdout)?.trim();
        let dest = "typescript.tgz";
        std::fs::rename(out, dest)?;
        Err("successful codegen. rerun without codegen.".into())
    }
}

const TYPESCRIPT_TGZ: &[u8] = include_bytes!("../typescript.tgz");

fn gunzip(bytes: &[u8]) -> Vec<u8> {
    let mut decoder = flate2::read::GzDecoder::new(std::io::Cursor::new(bytes));
    let mut data = Vec::new();
    use std::io::Read;
    decoder.read_to_end(&mut data).unwrap();
    data
}

fn tar_vfs(bytes: &[u8]) -> vfs::VfsPath {
    let fs: vfs::VfsPath = vfs::MemoryFS::new().into();
    let mut archive = tar::Archive::new(std::io::Cursor::new(bytes));
    for entry in archive.entries().unwrap() {
        let mut entry = entry.unwrap();
        let mut data = Vec::new();
        use std::io::Read;
        entry.read_to_end(&mut data).unwrap();
        let path = entry.path().unwrap().to_owned();
        let fs_path = fs.join(path.to_str().unwrap()).unwrap();
        fs_path.parent().create_dir_all().unwrap();
        let mut f = fs_path.create_file().unwrap();
        f.write_all(&data).unwrap();
    }
    fs
}

thread_local! {
    static TYPESCRIPT_FS: vfs::VfsPath = tar_vfs(&gunzip(TYPESCRIPT_TGZ)).join("package").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typescript_fs() -> Result<(), Box<dyn std::error::Error>> {
        TYPESCRIPT_FS.with(|fs| {
            let package_json = fs.join("package.json")?.read_to_string()?;
            let package_json: serde_json::Value = serde_json::from_str(&package_json)?;
            assert_eq!(
                package_json["name"].as_str().ok_or("not str")?,
                "typescript"
            );
            eprintln!("package_json.version={:?}", package_json["version"]);
            assert_eq!(
                package_json["main"].as_str().ok_or("not str")?,
                "./lib/typescript.js"
            );

            let typescript_js = fs.join("lib/typescript.js")?.read_to_string()?;
            eprintln!("typescript_js.len()={:?}", typescript_js.len());

            Ok(())
        })
    }
}
