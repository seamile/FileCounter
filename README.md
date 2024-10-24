# fcnt

![Crates.io](https://img.shields.io/crates/v/fcnt?color=9cf)
![Crates.io](https://img.shields.io/crates/d/fcnt?color=green&label=install)

**fcnt** is a file counter used in command line.

It can quickly count the number and size of huge amount of files in multiple directories through multi-threading.

## Usage

```shell
$ fcnt [OPTIONS] [DIRECTORIES]...
```

- Arguments:

  [DIRECTORIES]...  The directories (default: ./)

- Options:

  ```
  -a                   Count all regular and hidden files
  -d                   Count the number of directories
  -o <ORDER_BY>        The value to sort the results. Possible values: [n]ame, [f]ile, [d]ir, [s]ize.
  -r <PATTERN>         Match entries using regex (only matche filenames)
  -R                   Non-recursive mode (files in sub-directories will be ignored)
  -s                   Count the total size of files
  -t <TOP>             The number of threads for traversal (invalid in `non_recursive` mode)
  -T <THREAD_NUM>      The number of threads for traversal (invalid in `non_recursive` mode)
  -v                   Verbose mode, open this option will display the found entries
  -h, --help           Print help (see more with '--help')
  -V, --version        Print version
  ```

## Example

By default, only the number of sub-files in each directory is included in the result.

```shell
$ fcnt ./Pictures ./Movies ./Music
Path         Files
./Pictures/   3090
./Movies/     1658
./Music/      3606
──────────────────
Total         8354
```

You can use the `-d` option to show the number of subdirectories and the `-s` option to show the size of each directory.

```shell
$ fcnt -d -s ./Pictures ./Music ./src/package
Path            Files  Dirs   Size
./Pictures/      3090    37  18.1G
./Music/         3606  1285  21.8G
./src/package/  10458  3463   4.6G
──────────────────────────────────
Total           17154  4785  44.6G
```

The `-r` option can be used to filter files by regex and the `-o` option sorts the results in descending order on the specified column.

```shell
# Count the number of ".py" files in each subdirectory of "src" and sort by files.
$ fcnt -r '\.py$' -o f src/*
Path            Files
src/codebook/     829
src/package/      376
src/test/          31
src/spiderman/      7
src/Notes/          0
─────────────────────
Total            1243
```
