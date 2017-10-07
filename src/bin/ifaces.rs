extern crate ifaces;

#[cfg(not(test))]
fn main () {
    let ifaces = ifaces::Interface::get_all().unwrap()
                    .into_iter()
                    .filter(|x| x.kind != ifaces::Kind::Packet);
    for iface in ifaces {
        println!("{:?}", iface);
    }
}
