mod connection;
mod request;

use crate::connection::Connection;
use hugo::{HUGO, Image};
use std::net::{Ipv4Addr, TcpListener};

const PORT: u16 = 1337;

fn main() {
    let mut hugo = HUGO.lock().unwrap();

    let mut image = Image::default();

    let mut listener =
        TcpListener::bind((Ipv4Addr::UNSPECIFIED, PORT)).expect("unable to create tcp listener");

    listener
        .set_nonblocking(true)
        .expect("unable to configure tcp listener");

    let mut connections = Vec::new();

    loop {
        hugo.draw_image(image);

        connections.extend(Connection::try_accept(&mut listener));

        connections.retain_mut(|connection| match connection.request() {
            Some(request) => {
                if let Some(requested_image) = request.image() {
                    image = requested_image;
                }

                // TODO: schedule a programme switch

                false
            }
            None => true,
        });
    }
}
