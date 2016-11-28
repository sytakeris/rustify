extern crate reqwest;
extern crate serde_json;

use std::io::Read;
use std::vec::Vec;

use serde::de::Deserialize;
use self::reqwest::header::{Header, HeaderFormat, Authorization, Bearer};
use self::reqwest::Client;

use http::errors::HttpError;
use http::errors::Result;

pub struct Http;

impl Http {
    pub fn post<T, H>
    (uri: &str, param: Option<&Vec<(&str, String)>>, header: Option<H>) -> Result<T>
        where
            T: Deserialize,
            H: Header + HeaderFormat
    {

        let client = Client::new().unwrap();
        let mut builder = client.post(uri);

        builder = match param {
            Some(data) => builder.form(&data),
            None => builder
        };

        builder = match header {
            Some(data) => builder.header(data),
            None => builder
        };

        let mut response = builder.send()?;

        parse_response::<T>(&mut response)

    }

    pub fn get<T>
    (uri: &str, param: Option<Vec<(&str, &str)>>) -> Result<T>
        where
            T: Deserialize
    {
        let mut url = reqwest::Url::parse(uri).unwrap();

        if let Some(data) = param {
            url.query_pairs_mut().extend_pairs(data.iter());
        }

        let mut response = reqwest::get(url)?;

        parse_response::<T>(&mut response)

    }

    pub fn get_with_auth<T>
    (uri: &str, param: Option<Vec<(&str, &str)>>, token: &str) -> Result<T>
        where
            T: Deserialize
    {

        let mut url = reqwest::Url::parse(uri).unwrap();

        if let Some(data) = param {
            url.query_pairs_mut().extend_pairs(data.iter());
        }


        let client = Client::new().unwrap();
        let builder = client.get(uri)
                            .header(Authorization(
                                Bearer {
                                    token: token.to_owned()
                                }
                            ));


        let mut response = builder.send()?;

        parse_response::<T>(&mut response)
    }
}

fn parse_response<T: Deserialize>(response: &mut reqwest::Response) -> Result<T> {
    let mut content = String::new();
    response.read_to_string(&mut content)?;

    match serde_json::from_str::<T>(&content) {
        Ok(result) => Ok(result),
        Err(error) => Err(HttpError::ParsingResponseFailure(error))
    }
}
