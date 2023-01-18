# fcnt

**fcnt** is a file counter used in command line.

It can quickly count the number and size of huge amount of files in multiple directories through multi-threading.

## Usage

```shell
$ fcnt [OPTIONS] [DIRECTORIES]...
```

- Arguments:

  [DIRECTORIES]...  the directories (default: ./)

- Options:
  * `-a`             count all regular and hidden files
  * `-s`             count the total size of files
  * `-R`             non-recursive mode (files in sub-directories will be ignored)
  * `-t N_THREAD`    the number of threads for traversal (invalid in `non_recursive` mode)
  * `-h, --help`     Print help information
  * `-V, --version`  Print version information

## Example

By default, the results will be sorted by file count in descending order.

```shell
fcnt ./Pictures ./Music ./src/package
Path           Files  Dirs
./src/package   8070  3120
./Pictures      7799   274
./Music         3455  1196
──────────────────────────
Total          19324  4590
```

If the `-s` option is specified, the sort column will be change to `Size`.

```shell
$ fcnt -s ./Pictures ./Music ./src/package
Path           Files  Dirs   Size
./Music         3455  1196    21G
./Pictures      7799   274  17.5G
./src/package   8070  3120     4G
─────────────────────────────────
Total          19324  4590  42.6G
```
