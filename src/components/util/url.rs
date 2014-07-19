/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::hashmap::HashMap;
use rust_url;
use rust_url::{Url, UrlParser};

/**
Create a URL object from a string. Does various helpful browsery things like

* If there's no current url and the path looks like a file then it will
  create a file url based of the current working directory
* If there's a current url and the new path is relative then the new url
  is based off the current url

*/
// TODO: about:failure->
pub fn try_parse_url(str_url: &str, base_url: Option<Url>) -> Result<Url, &'static str> {
    let mut parser = UrlParser::new();
    let parser = match base_url {
        Some(ref base) => &*parser.base_url(base),
        None => &parser,
    };
    parser.parse(str_url.as_slice())
}

pub fn parse_url(str_url: &str, base_url: Option<rust_url::Url>) -> rust_url::Url {
    // FIXME: Need to handle errors
    try_parse_url(str_url, base_url).ok().expect("URL parsing failed")
}


pub type UrlMap<T> = HashMap<rust_url::Url, T>;

pub fn url_map<T: Clone + 'static>() -> UrlMap<T> {
    HashMap::new()
}


pub fn is_image_data(uri: &str) -> bool {
    static types: &'static [&'static str] = &["data:image/png", "data:image/gif", "data:image/jpeg"];
    types.iter().any(|&type_| uri.starts_with(type_))
}


