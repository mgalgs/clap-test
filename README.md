# Why?

Currently outputs:

```
$ ./target/debug/clap-test --print hello
error: The following required arguments were not supplied:

USAGE:
	clap-test [FLAGS] --print <something>...

For more information try --help
```

and:

```
$ ./target/debug/clap-test --help
clap-test 

USAGE:
	clap-test [FLAGS] --print <something>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --print <something>...    Print stuff
```
