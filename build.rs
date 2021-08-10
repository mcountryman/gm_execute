use std::{
  env,
  error::Error,
  fs::{self, read_dir},
  path::{Path, PathBuf},
  process::{Command, Stdio},
};

fn main() -> Result<(), Box<dyn Error>> {
  let tools = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tools");

  let dd = tools.join("DepotDownloader/DepotDownloader.exe");
  let files = tools.join("files");
  let out = PathBuf::from("target/vendor");

  println!("cargo:rustc-link-search=native={}/bin", out.display());
  println!(
    "cargo:rustc-link-search=native={}/garrysmod/bin",
    out.display()
  );
  println!("cargo:rustc-cdylib-link-arg=-Wl,-rpath,./garrysmod/bin,-rpath,{}/bin,-rpath,{}/garrysmod/bin",
    out.display(),
    out.display()
  );
  println!("cargo:rustc-cdylib-link-arg=-l:lua_shared.so");
  println!("cargo:rustc-cdylib-link-arg=-l:libtier0.so");
  println!("cargo:rustc-cdylib-link-arg=-l:libvstdlib.so");
  println!("cargo:rustc-cdylib-link-arg=-l:libsteam_api.so");

  println!("cargo:rerun-if-changed=tools/files");

  download_depot(&dd, &files, 4022, &out)?;
  download_depot(&dd, &files, 4023, &out)?;

  Ok(())
}

fn download_depot(
  dd: &Path,
  files: &Path,
  depot: u32,
  out: &Path,
) -> Result<(), Box<dyn Error>> {
  // Update lua_shared libraries
  let tmp = out.join("tmp");
  let output = Command::new(dd)
    .args(&["-app", "4020"])
    .args(&["-depot", &depot.to_string()])
    .args(&["-filelist", files.to_str().unwrap()])
    .args(&["-dir", tmp.to_str().unwrap()])
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?
    .wait_with_output()?;

  //
  if !output.status.success() {
    panic!(
      "DepotDownloader failed with `{}`",
      String::from_utf8_lossy(&output.stdout)
    );
  }

  copy_binaries(&tmp, out)?;

  Ok(())
}

fn copy_binaries(from: &Path, to: &Path) -> std::io::Result<()> {
  for entry in read_dir(from)? {
    let entry = entry?;
    let entry_type = entry.file_type()?;
    let entry_name = entry.file_name();
    let entry_name = entry_name.to_string_lossy();

    if !entry_type.is_file() {
      copy_binaries(&entry.path(), to)?;
    }

    if !entry_name.ends_with(".dll") || !entry_name.ends_with(".so") {
      continue;
    }

    fs::copy(entry.path(), to.join(entry.file_name()))?;
  }

  Ok(())
}
