//TODO #![deny(warnings, missing_docs)]

//! Module `ebay` provides concrete implementations for `clickshop-search`
//! REST APIs and related utility functions.
//!
//! # Examples


use std;
use native_tls;



/// Calculates BASE64 credentials for accessing eBay API endpoints.
///
/// # Example
/// ```
/// use clickshop_search::base64_credentials;
///
/// let client_id     = "username";
/// let client_secret = "password";
/// let credentials   = "dXNlcm5hbWU6cGFzc3dvcmQ=";
/// assert_eq!(credentials, base64_credentials(client_id, client_secret));
/// ```
pub fn base64_credentials(username: &str, password: &str) -> String {
    use base64;
    base64::encode(&(username.to_string() + ":" + password))
}

//---


/// Returns an HTTP body for retrieving client_credentials from eBay API endpoint.
///
/// # Example
/// ```
/// use clickshop_search::oauth_body;
///
/// let name = "myapp";
/// let body = "grant_type=client_credentials&redirect_uri=myapp&scope=https://api.ebay.com/oauth/api_scope";
/// assert_eq!(body, oauth_body(name));
/// ```
pub fn oauth_body(name: &str) -> String {
    format!("grant_type=client_credentials&redirect_uri={}&scope=https://api.ebay.com/oauth/api_scope", name)
}


/// Returns an HTTP header for retrieving client_credentials from eBay API endpoint.
///
/// # Example
/// ```
/// use clickshop_search::oauth_header;
/// use clickshop_search::oauth_body;
///
/// let host        = "api.sandbox.ebay.com";
/// let path        = "/identity/v1/oauth2/token";
/// let credentials = "These are wrong credentials, sorry!=";
/// let body        = oauth_body("username");
/// let expected    = "POST /identity/v1/oauth2/token HTTP/1.1\r\nHost: api.sandbox.ebay.com\r\nUser-Agent: curl/7.52.1\r\nContent-Type: application/x-www-form-urlencoded\r\nAuthorization: Basic dXNlcm5hbWU6cGFzc3dvcmQ=\r\nContent-Length: 94\r\n\r\n";
/// assert_eq!(expected, oauth_header(host, path, credentials, body.as_ref()));
/// ```
pub fn oauth_header(host: &str, path: &str, credentials: &str, body: &str) -> String {
    format!("\
            POST {} HTTP/1.1\r\n\
            Host: {}\r\n\
            User-Agent: curl/7.52.1\r\n\
            Content-Type: application/x-www-form-urlencoded\r\n\
            Authorization: Basic {}\r\n\
            Content-Length: {}\r\n\
            \r\n\
            ", path, host, credentials, body.len())
}


/// Returns an HTTP header for retrieving client_credentials from eBay API endpoint.
///
/// # Example
/// ```
/// use clickshop_search::oauth_header;
/// use clickshop_search::oauth_body;
/// use clickshop_search::oauth_token;
///
/// let host        = "api.sandbox.ebay.com";
/// let port        = 443;
/// let credentials = "These are wrong credentials, sorry!=";
/// let body        = oauth_body("username");
/// let header      = oauth_header("api.sandbox.ebay.com", "/identity/v1/oauth2/token", credentials, body.as_ref());
/// assert_eq!("XXXX", oauth_token(host, port, header.as_ref(), body.as_ref()));
/// ```
pub fn oauth_token(host: &str, port: u16, header: &str, body: &str) -> String {
    use futures::Future;
    use std::net::ToSocketAddrs;
    use tokio_io;
    use tokio_core;
    use tokio_tls::TlsConnectorExt;

    use io_dump::IoDump;

    let address = format!("{}:{}", host, port).to_socket_addrs().unwrap().next().unwrap();
    let data    = format!("{}{}", header, body);

    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle   = core.handle();
    let client   = tokio_core::net::TcpStream::connect(&address, &handle);

    let handshake = client
        .and_then(move |socket| {
            let cx  = native_tls::TlsConnector::builder().unwrap().build().unwrap();
            let tls = cx.connect_async(host, socket).map_err(native2io);
            tls
        });

    let wrapper = handshake
        .and_then(|socket| IoDump::wrapper(socket, std::path::Path::new("iodump.log")));

    let request = wrapper
        .and_then(|socket     | tokio_io::io::write_all(socket, data.as_bytes()))
        .and_then(|(socket, _)| tokio_io::io::flush(socket));

    let response = request
        .and_then(|socket| tokio_io::io::read_to_end(socket, Vec::new()));

    let (_, raw) = core.run(response).unwrap();
    assert!(raw.starts_with(b"HTTP/1.1 "));
    assert!(raw.ends_with(b"}"));
    let json = format!("{}", String::from_utf8_lossy(&raw));
    json
}

fn native2io(e: native_tls::Error) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, e)
}
