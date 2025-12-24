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
fn set_without_overwrite_fails_on_existing() {
    let d = tempdir().unwrap();
    let path = d.path().join("store.bin").to_str().unwrap().to_string();

    // initial set
    run_cmd_with_store(&["set", "dup", "v1"], &path).success();
    // second set without --overwrite should fail
    run_cmd_with_store(&["set", "dup", "v2"], &path)
        .failure()
        .stderr(predicate::str::contains("already exists"));

    // with --overwrite it should succeed
    run_cmd_with_store(&["set", "dup", "v2", "--overwrite"], &path)
        .success()
        .stdout(predicate::str::contains("Set 'dup'"));
}
