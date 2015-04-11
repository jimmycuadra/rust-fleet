# fleet

A Rust client library for [fleet](https://github.com/coreos/fleet).

## Known issues:

* Paginated resources are currently not handled. (Only the first page is returned.)

## Running the tests

The test suite includes integration tests that assume the fleet API to be running on localhost:2999. A Vagrant environment for this is provided. Simply follow these steps:

1. Install Vagrant.
1. `vagrant up`
1. `vagrant ssh`
1. `cd share`
1. `docker run -it --rm -v $(pwd):/source --net host jimmycuadra/rust`
1. `cargo test`

# License

[MIT](http://opensource.org/licenses/MIT)
