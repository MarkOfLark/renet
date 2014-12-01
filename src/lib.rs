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

pub mod ffi;


pub fn linked_version() -> u32 {
    unsafe {
        ffi::enet_linked_version() as u32
    }
}
