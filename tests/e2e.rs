use std::{
    fs::{File, create_dir_all, read_to_string},
    io::Write,
};

use tempfile::TempDir;
use toml::{Table, Value};
use trycmd::TestCases;
use which::which;
use xdg::BaseDirectories;

#[test]
fn e2e() {
    let tmp = TempDir::new().unwrap();
    let tmp = tmp.path();
    let dir = tmp.join("nix-init");

    create_dir_all(&dir).unwrap();

    let mut cfg: Table = toml::from_str(include_str!("default-config.toml")).unwrap();
    if let Some(tokens) = access_tokens() {
        cfg.insert("access-tokens".into(), tokens);
    }
    File::create(dir.join("config.toml"))
        .unwrap()
        .write_all(toml::to_string(&cfg).unwrap().as_bytes())
        .unwrap();

    TestCases::new()
        .default_bin_name("nix-init")
        .register_bin("mv", which("mv"))
        .env("XDG_CONFIG_HOME", tmp.to_str().unwrap())
        .case("tests/cmd/**/*.md")
        .case("tests/cmd/**/*.toml");
}

fn access_tokens() -> Option<Value> {
    let path = BaseDirectories::with_prefix("nix-init").find_config_file("config.toml")?;
    toml::from_str::<Table>(&read_to_string(path).ok()?)
        .ok()?
        .remove("access-tokens")
}
