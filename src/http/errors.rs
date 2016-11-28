extern crate reqwest;
extern crate serde_json;

use std;
//Request(reqwest::Error),
//Io(io::Error),
//Url(reqwest::UrlError),
//Parse(serde_json::Error),
quick_error! {
    #[derive(Debug)]
    pub enum HttpError {
        HttpRequestFailure(error: reqwest::Error) {
            from(e: reqwest::Error) -> (e)
        }

        HttpRequestIOFailure(error: std::io::Error) {
            from(e: std::io::Error) -> (e)
        }

        ParsingResponseFailure(error: serde_json::Error) {
            from(e: serde_json::Error) -> (e)
        }
    }
}

use std::result;
pub type Result<T> = result::Result<T, HttpError>;