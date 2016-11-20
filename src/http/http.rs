extern crate reqwest;
extern crate serde_json;

use std::io::Read;
use std::vec::Vec;

use serde::de::Deserialize;
use self::reqwest::header::{Header, HeaderFormat};
use self::reqwest::Client;

pub struct Http;

impl Http {
    pub fn post<T, H>
    (uri: &str, param: Option<&Vec<(&str, String)>>, header: Option<H>) -> T
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

        let mut response = builder.send().unwrap();

        let mut content = String::new();
        response.read_to_string(&mut content).unwrap();

        serde_json::from_str::<T>(&content).unwrap()

    }

    pub fn get<T>
    (uri: &str, param: Option<&Vec<(&str, String)>>) -> T
        where
            T: Deserialize
    {

        let client = Client::new().unwrap();
        let mut builder = client.get(uri);

        builder = match param {
            Some(data) => builder.form(&data),
            None => builder
        };

        let mut response = builder.send().unwrap();

        let mut content = String::new();
        response.read_to_string(&mut content).unwrap();

        serde_json::from_str::<T>(&content).unwrap()

    }
}