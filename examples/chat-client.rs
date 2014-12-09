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
extern crate serialize;

use serialize::json;
use std::io;

fn main() {
    let v = renet::linked_version();
    println!("the ENet library version is {}",v);

    match renet::initialize() {
        Ok(_)     => { println!("ENet is initialized!"); },
        Err(code) => { println!("ENet initialize returned error code {}",code); }
    }

    let client = match renet::Host::new(None::<&str>,1,2,0,0) {
        Ok(s)     => s, 
        Err(desc) => { println!("Error when trying to create client host: {}",desc); return; }
    };

    let peer = match client.connect("localhost:1234",2,None) {
        Ok(p)     => p,
        Err(desc) => { println!("Error when trying to connect to server: {}",desc); return; }
    };

    loop {
        let next_event = client.service(50000);
        match next_event {
            renet::Event::Connect(peer,packet) => { 
                println!("Connection Event: peer at address {}",peer.address);
            }
            renet::Event::Receive(peer,packet) => {
                println!("Receive Event: message from server {} is: {}",peer.address,json::decode(packet.data));
            }
            renet::Event::Disconnect(peer) => {
                println!("Disconnect Event: {} disconnected",peer);
            }
            None      => {}, // don't do anything if no events come in
        }

        println!("Input: ");
        let mut reader = io::stdin();
        let input = reader.read_line().ok().expect("Failed to read line");

        let packet = renet::Packet::new(json::encode(input),renet::Packet::RELIABLE).ok().expect("Could not create packet");
        peer.send(0,packet);
    }

    renet::deinitialize();
}
