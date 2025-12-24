use npass::store::Store;
use tempfile::tempdir;

#[test]
fn roundtrip_store_serialize() {
    let d = tempdir().unwrap();
    let path = d.path().join("store.bin");

    let mut s = Store::default();
    s.set("foo", "bar", true).expect("set foo");
    s.save_to_path(&path).expect("save");

    let s2 = Store::load_from_path(&path).expect("load");
    assert_eq!(s2.get("foo").map(|e| e.value.as_str()), Some("bar"));
}

#[test]
fn rm_moves_to_backups_and_force_deletes() {
    let d = tempdir().unwrap();
    let path = d.path().join("store.bin");

    // add alice
    let mut s = Store::default();
    s.set("alice", "secret1", true).expect("set alice");
    s.save_to_path(&path).expect("save alice");

    // load and remove non-force (should move to backups)
    let mut s2 = Store::load_from_path(&path).expect("load");
    let e = s2.remove("alice");
    assert!(e.is_some(), "expected alice to be present before remove");
    s2.backup_entry(e.unwrap());
    s2.save_to_path(&path).expect("save after backup");

    let s3 = Store::load_from_path(&path).expect("load after backup");
    assert!(
        s3.get("alice").is_none(),
        "alice should no longer be in entries"
    );
    assert!(
        s3.backups.iter().any(|b| b.name == "alice"),
        "alice should be in backups"
    );

    // now set and force delete
    let mut s4 = Store::load_from_path(&path).expect("load for bob");
    s4.set("bob", "secret2", true).expect("set bob");
    s4.save_to_path(&path).expect("save bob");

    let mut s5 = Store::load_from_path(&path).expect("load bob");
    let e = s5.remove("bob");
    assert!(e.is_some(), "expected bob to exist before remove");

    // force: do not backup
    s5.save_to_path(&path).expect("save after bob remove");
    let s6 = Store::load_from_path(&path).expect("load after bob remove");
    assert!(s6.get("bob").is_none(), "bob should be deleted");
    assert!(
        !s6.backups.iter().any(|b| b.name == "bob"),
        "bob should not be in backups"
    );
}
