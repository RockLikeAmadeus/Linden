Linden is a simple grep clone, in progress.

Run with

```bash
$ ./linden somestring example-file.txt
```

Or simply:

```bash
$ cargo run -- somestring example-file.txt
```

Decide whether to ignore case by setting (or not setting) the `IGNORE_CASE` environment variable.

```bash
$ IGNORE_CASE=1 cargo run -- somestring example-file.txt
```