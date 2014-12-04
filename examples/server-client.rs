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
 * -------------------------------------------------------------------
 */

extern crate renet;



fn main() {
    let v = renet::linked_version();
    println!("the ENet library version is {}",v);

    match renet::initialize() {
        Ok(_)     => { println!("ENet is initialized!"); },
        Err(code) => { println!("ENet initialize returned error code {}",code); }
    }

    let server = match renet::Host::new(Some("0.0.0.0:12345"),32,2,0,0) {
        Ok(s)     => s,
        Err(desc) => { println!("Server error: {}",desc); return; }
    };

    let client = match renet::Host::new(None::<&str>,1,2,57600,14400) {
        Ok(c)     => c,
        Err(desc) => { println!("Client error: {}",desc); return; }
    };

    renet::deinitialize();
}
