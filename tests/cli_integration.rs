use assert_cmd::cargo;
use predicates::prelude::*;
use tempfile::tempdir;

fn run_cmd_with_store(args: &[&str], store_path: &str) -> assert_cmd::assert::Assert {
    let mut cmd = cargo::cargo_bin_cmd!("npass");
    cmd.env("NPASS_STORE", store_path);
    cmd.args(args);
    cmd.assert()
}

#[test]
fn cli_set_get_ls_flow() {
    let d = tempdir().unwrap();
    let path = d.path().join("store.bin").to_str().unwrap().to_string();
    // set a password with positional value
    run_cmd_with_store(&["set", "name", "hunter2"], &path)
        .success()
        .stdout(predicate::str::contains("Set 'name'"));

    // get it
    run_cmd_with_store(&["get", "name"], &path)
        .success()
        .stdout(predicate::str::contains("hunter2"));

    // ls should list the name
    run_cmd_with_store(&["ls"], &path)
        .success()
        .stdout(predicate::str::contains("name"));
}

#[test]
fn rm_and_force_behaviour() {
    let d = tempdir().unwrap();
    let path = d.path().join("store.bin").to_str().unwrap().to_string();
    // set
    run_cmd_with_store(&["set", "a", "v1"], &path).success();
    // rm (backup)
    run_cmd_with_store(&["rm", "a"], &path)
        .success()
        .stdout(predicate::str::contains("Moved 'a' to backups"));

    // set bob and force remove
    run_cmd_with_store(&["set", "bob", "v2"], &path).success();
    run_cmd_with_store(&["rm", "bob", "--force"], &path)
        .success()
        .stdout(predicate::str::contains("Removed 'bob' permanently"));
}
