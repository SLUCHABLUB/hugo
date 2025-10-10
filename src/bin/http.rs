use hugo::{HUGO, Image};
use serde::Deserialize;
use std::io::{self, BufRead, BufReader, ErrorKind};
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use toml::from_str;

const PORT: u16 = 1337;

const HEADER_BODY_SEPARATOR: &str = "\r\n\r\n";

#[derive(Deserialize)]
#[serde(untagged)]
enum Number {
    Integer(i64),
    Float(f64),
}

#[derive(Deserialize)]
struct Request {
    image: Option<Image>,
    duration_in_seconds: Option<Number>,
    programme: Option<String>,
}

fn process(stream: &mut BufReader<TcpStream>) -> io::Result<Option<Request>> {
    match stream.fill_buf() {
        Ok(buffer) => {
            let request = str::from_utf8(buffer).map_err(io::Error::other)?;

            let (header, body) = request.split_once(HEADER_BODY_SEPARATOR).ok_or(io::Error::other("no blank line in http request"))?;

            // TODO: should we process the status line and headers?
            let _ = header;

            from_str(body).map_err(io::Error::other)
        }
        Err(error) if error.kind() == ErrorKind::WouldBlock => Ok(None),
        Err(error) => Err(error),
    }
}

fn try_accept_connection(listener: &mut TcpListener) -> Option<BufReader<TcpStream>> {
    let stream = listener.accept().and_then(|(stream, _address)| {
        stream.set_nonblocking(true)?;
        Ok(stream)
    });

    match stream {
        Ok(stream) => Some(BufReader::new(stream)),
        Err(error) if error.kind() == ErrorKind::WouldBlock => None,
        Err(error) => {
            eprintln!("Error when accepting new connection: {error}.");
            None
        }
    }
}

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

        connections.extend(try_accept_connection(&mut listener));

        connections.retain_mut(|connection| match process(connection) {
            Ok(Some(Request {
                image: requested_image,
                duration_in_seconds,
                programme,
            })) => {
                if let Some(requested_image) = requested_image {
                    image = requested_image;
                }

                // We can't switch programmes here.
                let _ = (duration_in_seconds, programme);

                false
            }
            Ok(None) => true,
            Err(error) => {
                eprintln!("Error when process connection: {error}.");
                false
            }
        });
    }
}
