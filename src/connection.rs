use crate::request::Request;
use std::io::{self, BufRead, BufReader, ErrorKind, Write};
use std::net::{TcpListener, TcpStream};
use toml::from_str;

const OK: &str = "200 OK";
const BAD_REQUEST: &str = "400 Bad Request";

const HEADER_BODY_SEPARATOR: &str = "\r\n\r\n";

pub struct Connection {
    stream: BufReader<TcpStream>,
    error: Option<io::Error>,
}

impl Connection {
    pub fn try_accept(listener: &mut TcpListener) -> Option<Connection> {
        let stream = listener.accept().and_then(|(stream, _address)| {
            stream.set_nonblocking(true)?;
            Ok(stream)
        });

        match stream {
            Ok(stream) => Some(Connection {
                stream: BufReader::new(stream),
                error: None,
            }),
            Err(error) if error.kind() == ErrorKind::WouldBlock => None,
            Err(error) => {
                eprintln!("Error when accepting new connection: {error}.");
                None
            }
        }
    }

    pub fn request(&mut self) -> Option<Request> {
        match self.try_request() {
            Ok(request) => request,
            Err(error) => {
                self.error = Some(error);
                None
            }
        }
    }

    fn try_request(&mut self) -> io::Result<Option<Request>> {
        match self.stream.fill_buf() {
            Ok(buffer) => {
                let request = str::from_utf8(buffer).map_err(io::Error::other)?;

                let (header, body) = request
                    .split_once(HEADER_BODY_SEPARATOR)
                    .ok_or(io::Error::other("no blank line in http request"))?;

                // TODO: should we process the status line and headers?
                let _ = header;

                from_str(body).map_err(io::Error::other)
            }
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => Ok(None),
            Err(error) => Err(error),
        }
    }

    fn respond(&mut self, status: &'static str, body: &str) -> io::Result<()> {
        let response = format_args!("HTTP/1.1 {status}{HEADER_BODY_SEPARATOR}{body}");

        self.stream.get_mut().write_fmt(response)
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        // Not our problem it the user doesn't get the response :P
        let _result = if let Some(error) = self.error.take() {
            self.respond(BAD_REQUEST, &error.to_string())
        } else {
            self.respond(OK, "")
        };
    }
}
