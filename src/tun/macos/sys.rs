use libc::{c_int, c_char, c_uchar, sockaddr, c_short, c_ushort, c_void, c_uint};

pub const IFNAMSIZ: usize = 16;

pub const IFF_UP: c_short = 0x1;
pub const IFF_RUNNING: c_short = 0x40;

pub const AF_SYS_CONTROL: c_ushort = 2;
pub const AF_SYSTEM: c_uchar = 32;
pub const PF_SYSTEM: c_int = AF_SYSTEM as c_int;
pub const SYSPROTO_CONTROL: c_int = 2;
pub const UTUN_OPT_IFNAME: c_int = 2;
pub const UTUN_CONTROL_NAME: &'static str = "com.apple.net.utun_control";


#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_info {
    pub ctl_id: c_uint,
    pub ctl_name: [c_uchar; 96],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_ctl {
    pub sc_len: c_uchar,
    pub sc_family: c_uchar,
    pub ss_sysaddr: c_ushort,
    pub sc_id: c_uint,
    pub sc_unit: c_uint,
    pub sc_reserved: [c_uint; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifdevmtu {
    pub current: c_int,
    pub min: c_int,
    pub max: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union _ifk_data {
    pub ptr: *mut c_void,
    pub value: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifkpi {
    pub ifk_module_id: c_uint,
    pub ifk_type: c_uint,
    pub ifk_data: _ifk_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union _ifr_ifru {
    pub ifr_addr: sockaddr,
    pub ifr_dstaddr: sockaddr,
    pub ifr_broadaddr: sockaddr,

    pub ifr_flags: c_short,
    pub ifr_metric: c_int,
    pub ifr_mtu: c_int,
    pub ifr_phys: c_int,
    pub ifr_media: c_int,
    pub ifr_intval: c_int,
    pub ifr_data: *mut c_void,
    pub ifr_devmtu: ifdevmtu,
    pub ifr_wake_flags: c_uint,
    pub ifr_route_refcnt: c_uint,
    pub ifr_reqcap: [c_int; 1],
    pub ifr_curcap: [c_int; 1],
    pub ifr_functional_type: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifreq {
    pub ifr_name: [c_char; IFNAMSIZ],
    pub ifr_ifru: _ifr_ifru,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifaliasreq {
    pub ifra_name: [c_char; IFNAMSIZ],
    pub ifra_addr: sockaddr,
    pub ifra_broadaddr: sockaddr,
    pub ifra_mask: sockaddr,
}

ioctl!(readwrite ctliocginfo with 'N', 3; ctl_info);

ioctl!(write siocsifflags with 'i', 16; ifreq);
ioctl!(readwrite siocgifflags with 'i', 17; ifreq);

ioctl!(write siocsifaddr with 'i', 12; ifreq);
ioctl!(readwrite siocgifaddr with 'i', 33; ifreq);

ioctl!(write siocsifdstaddr with 'i', 14; ifreq);
ioctl!(readwrite siocgifdstaddr with 'i', 34; ifreq);

ioctl!(write siocsifbrdaddr with 'i', 19; ifreq);
ioctl!(readwrite siocgifbrdaddr with 'i', 35; ifreq);

ioctl!(write siocsifnetmask with 'i', 22; ifreq);
ioctl!(readwrite siocgifnetmask with 'i', 37; ifreq);

ioctl!(write siocsifmtu with 'i', 52; ifreq);
ioctl!(readwrite siocgifmtu with 'i', 51; ifreq);

ioctl!(write siocaifaddr with 'i', 26; ifaliasreq);
ioctl!(write siocdifaddr with 'i', 25; ifreq);
