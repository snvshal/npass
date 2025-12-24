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
fn backup_rm_and_ls_flow() {
    let d = tempdir().unwrap();
    let path = d.path().join("store.bin").to_str().unwrap().to_string();

    // set then rm to backups
    run_cmd_with_store(&["set", "temp", "val1"], &path).success();
    run_cmd_with_store(&["rm", "temp"], &path)
        .success()
        .stdout(predicate::str::contains("Moved 'temp' to backups"));

    // backup ls should show the entry
    run_cmd_with_store(&["backup", "ls"], &path)
        .success()
        .stdout(predicate::str::contains("temp:val1"));

    // backup rm should remove it
    run_cmd_with_store(&["backup", "rm", "temp"], &path)
        .success()
        .stdout(predicate::str::contains(
            "Removed 'temp' from backups permanently",
        ));

    // backup ls should not show it anymore
    run_cmd_with_store(&["backup", "ls"], &path)
        .success()
        .stdout(predicate::str::contains("temp").not());
}

#[test]
fn backup_restore_flow() {
    let d = tempdir().unwrap();
    let path = d.path().join("store.bin").to_str().unwrap().to_string();

    // set then rm to backups
    run_cmd_with_store(&["set", "temp2", "val2"], &path).success();
    run_cmd_with_store(&["rm", "temp2"], &path)
        .success()
        .stdout(predicate::str::contains("Moved 'temp2' to backups"));

    // restore should move it back into entries
    run_cmd_with_store(&["backup", "restore", "temp2"], &path)
        .success()
        .stdout(predicate::str::contains("Restored 'temp2' from backups"));

    // get should now return the value
    run_cmd_with_store(&["get", "temp2"], &path)
        .success()
        .stdout(predicate::str::contains("val2"));

    // backup ls should not show it anymore
    run_cmd_with_store(&["backup", "ls"], &path)
        .success()
        .stdout(predicate::str::contains("temp2").not());
}
