# wc
Building my own version of the Unix command line tool wc.

Currently supports the options:
- -l (Shows the number of lines in a given file)
- -w (Shows the number of words in a given file)
- -c (Shows the number of bytes in a given file)
- -m (Shows the number of chars in a given file)

You can run the cli by running:
- cargo run -- -option  test.txt 

**note**: I'm learning Rust at the moment by making small projects like this one, which is the reason behind the beautiful amount of unwrap() and other weird lines of code.
