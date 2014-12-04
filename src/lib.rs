/*
 *           _           _            _             _          _
 *          /\ \        /\ \         /\ \     _    /\ \       /\ \
 *         /  \ \      /  \ \       /  \ \   /\_\ /  \ \      \_\ \
 *        / /\ \ \    / /\ \ \     / /\ \ \_/ / // /\ \ \     /\__ \
 *       / / /\ \_\  / / /\ \_\   / / /\ \___/ // / /\ \_\   / /_ \ \
 *      / / /_/ / / / /_/_ \/_/  / / /  \/____// /_/_ \/_/  / / /\ \ \
 *     / / /__\/ / / /____/\    / / /    / / // /____/\    / / /  \/_/
 *    / / /_____/ / /\____\/   / / /    / / // /\____\/   / / /
 *   / / /\ \ \  / / /______  / / /    / / // / /______  / / /
 *  / / /  \ \ \/ / /_______\/ / /    / / // / /_______\/_/ /
 *  \/_/    \_\/\/__________/\/_/     \/_/ \/__________/\_\/
 *
 *
 * This file is released under terms described in the LICENSE file at
 * the top directory of this repository.
 *
 * Please contact Mark McDermott <mark.elias.mcdermott@gmail.com> or
 * the current maintainer of this software if you do not have a copy of
 * the license file.
 *
 * -------------------------------------------------------------------
 *  REnet is a rust language wrapper for the ENet networking library
 *
 *  TODO List
 * -------------------------------------------------------------------
 */

extern crate libc;

use libc::{c_uint,c_void,size_t};
use std::io::net::ip::ToSocketAddr;
use std::io::net::ip::IpAddr;

pub mod ffi;


pub fn initialize() -> Result<(), i32>  {
    unsafe {
        match ffi::enet_initialize() {
            0    => Ok(()),
            code => Err(code)
        }
    }
}

pub fn deinitialize() {
    unsafe {
        ffi::enet_deinitialize();
    }
}


pub fn linked_version() -> u32 {
    unsafe {
        ffi::enet_linked_version() as u32
    }
}


/*
 * ------------------------------------------------------------------
 * Host object: a wrapper around ENetHost*
 * ------------------------------------------------------------------
 */
pub struct Host {
    ffi_handle : *mut c_void,
}

impl Host {
    pub fn new<A: ToSocketAddr>(address: Option<A>,
                                peer_count: u32,
                                channel_count: u32,
                                incomming_bandwidth: u32,
                                outgoing_bandwidth: u32)
    -> Result<Host, &'static str> {

        let p_addr = match address {
            Some(addr) => {
                let socket_addr = match addr.to_socket_addr() {
                    Ok(a)      => a,
                    Err(ioerr) => return Err(ioerr.desc)
                };

                let socket_ip : u32 = match socket_addr.ip {
                    IpAddr::Ipv4Addr(a,b,c,d) => (a as u32 <<24) + (b as u32 <<16) + (c as u32 <<8) + d as u32,
                    _                         => return Err("IPv6 not currently supported")
                };
                let eaddr = ffi::ENetAddress{host:socket_ip, port:socket_addr.port};
                &eaddr as *const _ as *const c_void
            }
            None => 0 as *const c_void
        };

        let host = unsafe { ffi::enet_host_create(p_addr,
                                                  peer_count as size_t,
                                                  channel_count as size_t,
                                                  incomming_bandwidth as c_uint,
                                                  outgoing_bandwidth as c_uint)
                          };

        let p_host = host as *mut Host;

        if p_host.is_null() { return Err("Could not initialize host"); }
        else                { return Ok(Host{ffi_handle:host}); }
    }

    pub fn hello(&self) {
        println!("hi");
    }
}

impl Drop for Host {
    fn drop(&mut self) {
        println!("deleting host");

        unsafe { ffi::enet_host_destroy(self.ffi_handle); }
    }
}
