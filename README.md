# fcnt

**fcnt** is a file counter used in command line.

It can quickly count the number and size of huge amount of files in multiple directories through multi-threading.

## Usage

```shell
$ fcnt3 [OPTIONS] [DIRECTORIES]...
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

```shell
$ fcnt -as ./Pictures ./Music ./Downloads/Gzh/Gifts

Name               Files  Dirs    Size
./Pictures          7783   276   17.6G
./Music             3671  1199     21G
./Downloads/Gifts    295     6  689.8M
```

## TODO

- [ ] add test cases.
- [ ] add documentation in the source code.
