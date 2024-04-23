# ignore_unlink

Small experiment using [reverie](https://github.com/facebookexperimental/reverie) to capture unlink syscalls and ignore them.

Example:
```bash
➜ touch /tmp/file

➜ ./target/release/ignore_unlink rm -- -f /tmp/file
Unlinkat: tried to remove file "/tmp/file"

➜ stat /tmp/file
  File: /tmp/file
  Size: 0               Blocks: 0          IO Block: 4096   regular empty file
Device: 0,37    Inode: 12164       Links: 1
Access: (0644/-rw-r--r--)  Uid: ( 1000/  adrian)   Gid: ( 1000/  adrian)
Access: 2024-04-23 17:16:58.747868036 +0200
Modify: 2024-04-23 17:16:58.747868036 +0200
Change: 2024-04-23 17:16:58.747868036 +0200
 Birth: 2024-04-23 17:16:39.583880592 +0200
```

Could be useful to keep temporary files around for debugging purposes.
