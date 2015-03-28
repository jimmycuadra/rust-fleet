# fleet_client

A Rust client library for [fleet](https://github.com/coreos/fleet). Pre-pre-pre-alpha. Do not attempt to use.

## Running the tests

The test suite includes integration tests that assume the fleet API to be running on localhost:2999. A Vagrant environment for this is provided. Simply follow these steps:

1. Install Vagrant.
1. cp `user-data.sample user-data`
1. `vagrant up`
1. `vagrant ssh`
1. `cd share`
1. `docker run -it --rm -v $(pwd):/source --net host schickling/rust`
1. `cargo test`

# License

[MIT](http://opensource.org/licenses/MIT)
