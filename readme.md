# find_duplicates - Find Duplicates in a Directory Tree

`find_duplicates` is a tool to find duplicate files in a directory structure. It uses the MD5 digest to determine duplicate files. I might consider adding an option to use SHA-256 or something similar, but MD5 is good as a quick duplicate test.

I mostly use this for finding duplicates in legacy backup data. It was also a good excuse to try Rust as a console (CLI) tool builder.


## Usage

```
find_duplicates [OPTIONS] <PATH>

Arguments:
  <PATH>  The base path to search

Options:
  -j, --json <JSON_FILE>  Output json to a file
  -a, --all               Include all files (instead of only duplicates)
  -h, --help              Print help information

```

