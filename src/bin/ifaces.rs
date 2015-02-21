extern crate "rust-ifaces" as ifaces;

fn main () {
    for iface in
        ifaces::Interface::get_all().unwrap()
            .into_iter()
            .filter(|x| x.kind == ifaces::Kind::Packet) {
                println!("{}", iface.name);
            }
}
