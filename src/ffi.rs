use libc::{c_void, c_char, c_int, c_uint};
use std::{mem, net, ptr};
use nix::sys::socket;

// nix doesn't have this const
pub const AF_PACKET: i32 = 17;

#[allow(dead_code)]
#[repr(C)]
pub enum SIOCGIFFLAGS {
    IFF_UP = 0x1,		/* Interface is up.  */
    IFF_BROADCAST = 0x2,	/* Broadcast address valid.  */
    IFF_DEBUG = 0x4,		/* Turn on debugging.  */
    IFF_LOOPBACK = 0x8,		/* Is a loopback net.  */
    IFF_POINTOPOINT = 0x10,	/* Interface is point-to-point link.  */
    IFF_NOTRAILERS = 0x20,	/* Avoid use of trailers.  */
    IFF_RUNNING = 0x40,		/* Resources allocated.  */
    IFF_NOARP = 0x80,		/* No address resolution protocol.  */
    IFF_PROMISC = 0x100,	/* Receive all packets.  */

    /* Not supported */
    IFF_ALLMULTI = 0x200,	/* Receive all multicast packets.  */

    IFF_MASTER = 0x400,		/* Master of a load balancer.  */
    IFF_SLAVE = 0x800,		/* Slave of a load balancer.  */

    IFF_MULTICAST = 0x1000,	/* Supports multicast.  */

    IFF_PORTSEL = 0x2000,	/* Can set media type.  */
    IFF_AUTOMEDIA = 0x4000,	/* Auto media select active.  */
    IFF_DYNAMIC = 0x8000	/* Dialup device with changing addresses.  */
}

#[repr(C)]
pub struct union_ifa_ifu {
    pub data: *mut c_void,
}
impl union_ifa_ifu {
    pub fn ifu_broadaddr (&mut self) -> *mut socket::sockaddr {
        self.data as *mut socket::sockaddr
    }
    pub fn ifu_dstaddr (&mut self) -> *mut socket::sockaddr {
        self.data as *mut socket::sockaddr
    }
}

#[repr(C)]
pub struct ifaddrs {
    pub ifa_next: *mut ifaddrs,
    pub ifa_name: *mut c_char,
    pub ifa_flags: c_uint,
    pub ifa_addr: *mut socket::sockaddr,
    pub ifa_netmask: *mut socket::sockaddr,
    pub ifa_ifu: union_ifa_ifu,
    pub ifa_data: *mut c_void,
}

extern "C" {
    pub fn getifaddrs (ifap: *mut *mut ifaddrs) -> c_int;
    pub fn freeifaddrs (ifa: *mut ifaddrs) -> c_void;
}

pub fn convert_sockaddr (sa: *mut socket::sockaddr) -> Option<net::SocketAddr> {
    if sa == ptr::null_mut() { return None; }

    let (addr, port) = match unsafe { *sa }.sa_family as i32 {
        socket::AF_INET => {
            let sa: *const socket::sockaddr_in = unsafe { mem::transmute(sa) };
            let sa = & unsafe { *sa };
            let (addr, port) = (sa.sin_addr.s_addr, sa.sin_port);
            (
                net::IpAddr::new_v4(
                    ((addr & 0x000000FF) >>  0) as u8,
                    ((addr & 0x0000FF00) >>  8) as u8,
                    ((addr & 0x00FF0000) >> 16) as u8,
                    ((addr & 0xFF000000) >> 24) as u8,
                    ),
                port
            )
        },
        socket::AF_INET6 => {
            let sa: *const socket::sockaddr_in6 = unsafe { mem::transmute(sa) };
            let sa = & unsafe { *sa };
            let (addr, port) = (sa.sin6_addr.s6_addr, sa.sin6_port);
            (
                net::IpAddr::new_v6(
                    addr[0],
                    addr[1],
                    addr[2],
                    addr[3],
                    addr[4],
                    addr[5],
                    addr[6],
                    addr[7],
                    ),
                port
            )
        },
        _ => return None,
    };
    Some(net::SocketAddr::new(addr, port))
}
