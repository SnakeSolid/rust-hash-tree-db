# Hash Tree Database

Hash Tree Database (HTDB) is simple wrapper for in-memory key-value storage. It based on default implementation of
`HashMap` and `BTreeMap`, and provides simple way to save and load data from file.

Project provides to ways to communicate with database - CLI utility and JNI library.

NOTE: Swapping pages to disk not implemented yet.

## CLI Arguments

* `-m`, `--memory-pages` - number of pages with allowed to keep in memory. If number of pages will be greater than this number then old pages will be stored to disk;
* `-p`, `--page-size` - maximal number of entries per single page. If page contains more element than this limit then page will be splitted;
* `-s`, `--storage-path` - path to database storage directory. This directory will contains full database file and swapped pages.

## CLI commands

* GET `partition` `key` - get value from `partition` using `key`.
* PUT `partition` `key` `value` - get value from `partition` using `key`.
* CONTAINS `partition` `key` - check that`partition` contains given `key`.
* DELETE `partition` `key` - delete value pair from `partition` using `key`.
* RANGE `partition` `key_first` `key_last` - returns all key-value pairs in `partition` from `key_first` to `key_last`.
* SUCC `partition` `key` - returns key/value pair corresponding to next `key`.
* PRED `partition` `key` - returns key/value pair corresponding to previous `key`.
* COUNT - returns total number of values in database.
* SHOW - show full database content.
* SAVE - save database to local file in storage directory.
* LOAD - load database from local file in storage directory.
* EXIT - exit from CLI.

## CLI Session Example

```bash
$ cargo run --package htdb-cli
>> PUT 1 1 a
OK
>> SHOW
partition "1":
    page #0 ["1" .. "1"]: { "1" => "a", },
OK
>> PUT 1 1 b
OK
>> SHOW
partition "1":
    page #0 ["1" .. "1"]: { "1" => "b", },
OK
>> PUT 1 2 c
OK
>> PUT 1 3 d
OK
>> DELETE 1 1
OK TRUE
>> SHOW
partition "1":
    page #0 ["1" .. "3"]: { "2" => "c", "3" => "d", },
OK
>> EXIT
```

## License

This project is licensed under the MIT License.
