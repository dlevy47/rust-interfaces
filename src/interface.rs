use libc;
use std::ffi::CStr;
use std::{mem, net, ptr};

use nix;
use nix::sys::socket;

use ffi;

pub enum Kind {
    Packet,
    Ipv4,
    Ipv6,
}

pub enum NextHop {
    Broadcast(net::SocketAddr),
    Destination(net::SocketAddr),
}

pub struct Interface {
    /// The name of this interface.
    pub name: String,

    /// The kind of interface this is.
    pub kind: Kind,

    /// The address of this interface, if it has one.
    pub addr: Option<net::SocketAddr>,

    /// The netmask of this interface, if it has one.
    pub mask: Option<net::SocketAddr>,

    /// The broadcast address or destination address, if it has one.
    pub hop: Option<NextHop>,
}

impl Interface {
    /// Retrieve a list of interfaces on this system.
    pub fn get_all () -> Result<Vec<Interface>, nix::errno::Errno> {
        let mut ifap: *mut ffi::ifaddrs = unsafe { mem::zeroed() };
        if unsafe { ffi::getifaddrs(&mut ifap as *mut _) } != 0 {
            return Err(nix::errno::Errno::last());
        }

        let mut ret = Vec::new();
        let mut cur: *mut ffi::ifaddrs = ifap;
        while cur != ptr::null_mut() {
            if let Some(iface) = convert_ifaddrs(cur) {
                ret.push(iface);
            }
            //TODO: do something else maybe?
            cur = unsafe { (*cur).ifa_next };
        }

        unsafe { ffi::freeifaddrs(ifap) };

        Ok(ret)
    }
}

fn convert_ifaddrs (ifa: *mut ffi::ifaddrs) -> Option<Interface> {
    let ifa = unsafe { &mut *ifa };
    let name = match String::from_utf8(unsafe {
        CStr::from_ptr(ifa.ifa_name)
    }.to_bytes().to_vec()) {
        Ok(x) => x,
        Err(_) => return None,
    };

    let kind = if ifa.ifa_addr == ptr::null_mut() {
        match unsafe { *ifa.ifa_addr }.sa_family as i32 {
            ffi::AF_PACKET => Kind::Packet,
            socket::AF_INET => Kind::Ipv4,
            socket::AF_INET6 => Kind::Ipv6,
            _ => return None,
        }
    } else {
        return None;
    };

    let addr = ffi::convert_sockaddr(ifa.ifa_addr);

    let mask = ffi::convert_sockaddr(ifa.ifa_netmask);

    let hop = if ifa.ifa_flags & ffi::SIOCGIFFLAGS::IFF_BROADCAST as libc::c_uint == ffi::SIOCGIFFLAGS::IFF_BROADCAST as libc::c_uint {
        match ffi::convert_sockaddr(ifa.ifa_ifu.ifu_broadaddr()) {
            Some(x) => Some(NextHop::Broadcast(x)),
            None => None,
        }
    } else {
        match ffi::convert_sockaddr(ifa.ifa_ifu.ifu_dstaddr()) {
            Some(x) => Some(NextHop::Destination(x)),
            None => None,
        }
    };

    Some(Interface {
        name: name,
        kind: kind,
        addr: addr,
        mask: mask,
        hop: hop,
    })
}
