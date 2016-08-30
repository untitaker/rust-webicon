error_chain! {
    types {
        Error, ErrorKind, ChainErr, Result;
    }

    foreign_links {
        ::hyper::Error, Hyper;
        ::std::io::Error, Io;
        ::image::ImageError, Image;
    }

    errors {
        BadStatusCode(response: ::hyper::client::response::Response) {
            description("Bad status code")
            display("Bad status code: {}", response.status)
        }
        NoContentType(response: ::hyper::client::response::Response) {
            description("No Content-Type found.")
        }
        BadContentType(response: ::hyper::client::response::Response) {
            description("Invalid Content-Type for image.")
        }
    }
}
