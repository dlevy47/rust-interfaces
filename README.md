# Rust bindings to retrieve network interface information

This library contains functionality to retrieve network interface information on Linux machines.

## Example usage

See `src/bin/ifaces.rs` for an example of printing out network interfaces on a machine:

```bash
cargo run --bin ifaces
```

On my machine, this prints out:

```
Interface { name: "lo0", kind: Ipv4, addr: Some(V4(127.0.0.1:0)), mask: Some(V4(255.0.0.0:0)), hop: Some(Destination(V4(127.0.0.1:0))) }
Interface { name: "lo0", kind: Ipv6, addr: Some(V6([::]:0)), mask: Some(V6([ff:ff:ff:ff:ff:ff:ff:ff]:0)), hop: Some(Destination(V6([::]:0))) }
Interface { name: "lo0", kind: Ipv6, addr: Some(V6([fe:80::]:0)), mask: Some(V6([ff:ff:ff:ff:ff:ff:ff:ff]:0)), hop: None }
Interface { name: "en0", kind: Ipv6, addr: Some(V6([fe:80::]:0)), mask: Some(V6([ff:ff:ff:ff:ff:ff:ff:ff]:0)), hop: None }
Interface { name: "en0", kind: Ipv4, addr: Some(V4(192.168.0.101:0)), mask: Some(V4(255.255.255.0:0)), hop: Some(Broadcast(V4(192.168.0.255:0))) }
Interface { name: "awdl0", kind: Ipv6, addr: Some(V6([fe:80::]:0)), mask: Some(V6([ff:ff:ff:ff:ff:ff:ff:ff]:0)), hop: None }
Interface { name: "utun0", kind: Ipv6, addr: Some(V6([fe:80::]:0)), mask: Some(V6([ff:ff:ff:ff:ff:ff:ff:ff]:0)), hop: None }
```
