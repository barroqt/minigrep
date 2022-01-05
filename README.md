### USAGE
```
cargo run searchstring example-filename.txt
```
For case sensitive search

```
CASE_INSENSITIVE=1 cargo run searchstring example-filename.txt
```
For case insensitive search

### Development steps
1/ Two command line arguments: a string to search for and a filename.
    a) Read arguments
    b) Read file
    c) Refactore
    d) Error handling
    e) Split config and read logic code into crates
    f) Unit tests

2/ Search function to match the string queried and the file content
    a) Unit test (TDD)
    b) Implement in code logic
    c) Deal with case insensitive search
        - unit test
        - write function
        - setup config with env variable 
    d) App error printed on the error stream
