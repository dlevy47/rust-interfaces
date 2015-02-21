# Rust bindings to retrieve network interface information

This library contains functionality to retrieve network interface information on Linux machines.

## Example usage

See `src/bin/ifaces.rs`:

```rust
extern crate "rust-ifaces" as ifaces;

fn main () {
    for iface in
        ifaces::Interface::get_all().unwrap()
            .into_iter()
            .filter(|x| x.kind == ifaces::Kind::Packet) {
                println!("{}", iface.name);
            }
}
```

On my machine, this prints out:

```
$ target/ifaces
lo
eth0
wlan0
docker0
```
