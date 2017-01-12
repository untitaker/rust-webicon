error_chain! {
    types {
        Error, ErrorKind, ChainErr, Result;
    }

    foreign_links {
        ::reqwest::Error, Hyper;
        ::std::io::Error, Io;
        ::image::ImageError, Image;
    }

    errors {
        BadStatusCode(response: ::reqwest::Response) {
            description("Bad status code")
            display("Bad status code: {}", response.status())
        }
        NoContentType(response: ::reqwest::Response) {
            description("No Content-Type found.")
        }
        BadContentType(response: ::reqwest::Response) {
            description("Invalid Content-Type for image.")
        }
    }
}
