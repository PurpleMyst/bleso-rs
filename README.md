bleso
=====

`bleso` is a lisp interpreter written in Rust.

Installation
------------

Requirements

* Rust (click [here](https://www.rust-lang.org/en-US/install.html) for a guide
  on installing rust)

After having installed Rust and Cargo, you can compile with the following:

```shell
$ cargo build --release
[... cargo output ...]
```

Usage
-----

After having written some lisp code and saved it to a file (that for this
example we'll assume is called `example.cl`), you can interpret your code with
the following:

```shell
$ cargo run --release example.cl
[... your program's output ...]
```

Contributing
------------

Pull requests are welcome.
For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

License
-------

[MIT](https://choosealicense.com/licenses/mit/)
