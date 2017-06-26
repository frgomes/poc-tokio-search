[ABANDONWARE] Server side responsible for finding products according to user queries.

You may also find this project useful: https://github.com/frgomes/io-dump

**NOTE:** This is a proof of concept which employs Tokio for assynchronous I/O, consuming
data from eBay. Unfortunately, this effort is abandoned at this point since, in spite
the OAuth2 autentication works, the socket gets closed for unknown reasons.

In other words: DO NOT RELY on this code as it is at the moment.

## For the impatient

### Environment

Rust can be installed via `debian-scripts`:

```
$ cd $HOME/workspace
$ git clone http://github.com/frgomes/debian-bin \
  && sudo debian-bin/install-rust.sh
$ source $HOME/.cargo/env
```

### System dependencies

```
$ sudo apt-get install libssl-dev
```

### Building

```bash
$ git clone https://github.com/frgomes/poc-tokio-search
$ cd poc-tokio-ebay
$ cargo build
```

### Running

```bash
$ cargo run &
$ curl http://localhost:8080/200
```
