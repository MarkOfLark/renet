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
pub struct Host;

impl Host {
    pub fn new<'h>(address: u32, //TODO
                   peer_count: u32,
                   channel_count: u32,
                   incomming_bandwidth: u32,
                   outgoing_bandwidth: u32)
                   -> Result<&'h Host, &'static str>
    {
        let host = unsafe { ffi::enet_host_create(address as *const libc::c_void, //TODO
                                                  peer_count as libc::size_t,
                                                  channel_count as libc::size_t,
                                                  incomming_bandwidth as libc::c_uint,
                                                  outgoing_bandwidth as libc::c_uint)
                          } as *mut Host;

        if host.is_null() { return Err("Could not initialize host"); }
        else              { return Ok(unsafe{& *host}); }
    }

    pub fn drop(&mut self) {
        //ffi::enet_host_destroy(self.ffi_handle);
    }
}
