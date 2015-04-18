# fleet

A Rust client library for [fleet](https://github.com/coreos/fleet).

## Documentation

https://jimmycuadra.github.io/rust-fleet/fleet/

## Example

``` rust
extern crate fleet;

use fleet::Client;

let client = Client::new("http://localhost:2999");

match client.list_units() {
    Ok(unit_page) => {
        for unit in unit_page.units {
            println!("{}", unit.name);
        }
    },
    None => println!("No units in fleet!"),
};
```

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
