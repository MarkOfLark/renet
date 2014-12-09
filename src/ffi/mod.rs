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


const ENET_EVENT_TYPE_CONNECT: libc::c_uint    = 1;
const ENET_EVENT_TYPE_DISCONNECT: libc::c_uint = 2;
const ENET_EVENT_TYPE_receive: libc::c_uint    = 3;

#[repr(C)]
pub struct ENetAddress {
    pub host: libc::c_uint,
    pub port: libc::c_ushort,
}

#[repr(C)]
pub struct ENetEvent {
    pub etype: libc::c_uint,
    pub peer: *mut libc::c_void,
    pub channelID: libc::c_char,
    pub data: libc::c_uint,
    pub packet: *mut libc::c_void,
}

#[link(name = "enet")]
extern {
    pub fn enet_initialize() -> libc::c_int;
    pub fn enet_deinitialize();
    pub fn enet_linked_version() -> libc::c_uint;
    pub fn enet_host_create(address           : *const libc::c_void,
                            peerCount         : libc::size_t,
                            channelCount      : libc::size_t,
                            incomingBandwidth : libc::c_uint,
                            outgoingBandwidth : libc::c_uint) -> *mut libc::c_void;
    pub fn enet_host_service(host : *mut libc::c_void, event : *mut libc::c_void, timeout : libc::c_uint) -> libc::c_int;
    pub fn enet_host_destroy(host : *mut libc::c_void);
}
