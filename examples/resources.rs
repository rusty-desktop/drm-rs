extern crate drm;

/// Check the `util` module to see how the `Card` structure is implemented.
pub mod util;
use util::*;

pub fn main() {
    let card = Card::open_global();

    let resources = card.resource_handles().unwrap();
    let plane_res = card.plane_handles().unwrap();

    // Print out all card resource handles
    println!("Connectors:\t{:?}", resources.connectors());
    println!("Encoders:\t{:?}", resources.encoders());
    println!("CRTCs:\t\t{:?}", resources.crtcs());
    println!("Framebuffers:\t{:?}", resources.framebuffers());
    println!("Planes:\t\t{:?}", plane_res.planes());

    for &handle in resources.connectors() {
        println!("{:#?}", card.get_connector(handle));
    }
/*
    for &handle in resources.encoders() {
        println!("{:#?}", card.get_encoder(handle));
    }

    for &handle in resources.crtcs() {
        println!("{:#?}", card.get_crtc(handle));
    }

    for &handle in resources.framebuffers() {
        println!("{:#?}", card.get_framebuffer(handle));
    }

    for &handle in plane_res.planes() {
        println!("{:#?}", card.get_plane(handle));
    }
    */
}
