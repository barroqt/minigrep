# THE MINIGREP

## Usage For case sensitive search

`cargo run searchstring example-filename.txt`

## Usage For case insensitive search

`CASE_INSENSITIVE=1 cargo run searchstring example-filename.txt`

## Development steps

* __Two command line arguments: a string to search for and a filename.__
  * Read arguments
  * Read file
  * Refactore
  * Error handling
  * Split config and read logic code into crates
  * Unit tests

* __Search function to match the string queried with the file content__
  * Unit test (TDD)
  * Implement in code logic
  * Deal with case insensitive search
    * unit test
    * write function
    * setup config with env variable
  * App error printed on the error stream

* __Make code clearer with Iterators__
