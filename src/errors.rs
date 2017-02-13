error_chain! {
    types {
        Error, ErrorKind, ChainErr, Result;
    }

    foreign_links {
        Hyper(::reqwest::Error);
        Io(::std::io::Error);
        Image(::image::ImageError);
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
