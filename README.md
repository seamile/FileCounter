# fcnt

`fcnt` means "file counter".

It is a cmd-line tool for counting the number of files in given directories.

Directory traversal uses multiple threads.

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
  * `-t <N_THREAD>`  the number of threads for traversal (invalid in `non_recursive` mode)
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
