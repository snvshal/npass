# npass — simple password manager

**npass** is a minimal command-line password manager that supports storing simple name/value entries.

---

## Quick features

* Commands: `set`, `get`, `rm`, `ls`
* Backups: `rm` without `--force` moves entries to backups (so they can be recovered)
* Store format: cleartext JSON (easy to inspect and backup) — use external encryption if you need stronger protection

---

## Installation

* Build locally:

  ```bash
  cargo build --release
  ```
* Install via cargo:

  ```bash
  cargo install --path .
  ```

---

## Usage

### Set a password (positional)

```bash
npass set name hunter2
```

### Set interactively (prompt for value)

```bash
npass set name
```

### Set and overwrite existing entries

Add `--overwrite` to replace an existing entry (without it, `set` will fail if the name exists):

```bash
npass set name newpass --overwrite
```

### Get a password

```bash
npass get name
```

### List names

```bash
npass ls
```

### List all values & backups

```bash
npass ls --all
```

### Remove (move to backups)

```bash
npass rm name
```

### Remove permanently from entries

```bash
npass rm name --force
```

### Manage backups

```bash
npass backup ls              # list backups
npass backup rm name         # remove from backups permanently
npass backup restore name    # restore entry from backups into the store
```

---

## Security notes

* The store is currently saved as **cleartext JSON** (no encryption). Internally the file is written to the app data directory returned by `directories::ProjectDirs` and the filename is `store.bin` (i.e. `<data-dir>/store.bin`). On Linux this typically maps to `$XDG_DATA_HOME/com/example/npass/store.bin` or `~/.local/share/com/example/npass/store.bin` when `XDG_DATA_HOME` is not set.

* You can override the store path for testing or CI using the `NPASS_STORE` environment variable (e.g. `NPASS_STORE=/tmp/npass-store.bin`).

* If you need stronger protection, use filesystem-level encryption or an encrypted container. Adding optional built-in encryption (e.g., password-based AES-GCM) is straightforward and I can implement it on request.

---

## Contributing

* See `CONTRIBUTING.md` for guidelines. Open issues/PRs if you'd like features or fixes.

---

## License

* MIT — see `LICENSE` file.
