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

    server = match renet::Host::new(Some("localhost:1234"),32,2,0,0) {
        Ok(s)     => s, 
        Err(desc) => { println!("Error when trying to create server host: {}",desc); return;) }
    };

    loop {
        let next_event = server.service(50000);
        match next_event {
            renet::Event::Connect(peer,packet) => { 
                println!("Connection Event: new peer at address {}",peer.address);
            }
            renet::Event::Receive(peer,packet) => {
                println!("Receive Event: message from client {} is {}",peer.address,json::decode(packet.data));
                server.broadcast(0,packet); // echo the message to all other clients
            }
            renet::Event::Disconnect(peer) => {
                println!("Disconnect Event: client {} disconnected",peer);
            }
            None      => {}, // don't do anything if no events come in
        }
    }

    renet::deinitialize();
}
