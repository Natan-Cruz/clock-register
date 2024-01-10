use std::{collections::HashMap, ops::Add};
use regex::Regex;
use url::Url;
use http;

mod http_result;
use http_result::HttpResult;


mod http_input_stream;
use http_input_stream::HttpInputStream;

mod http_request;
use http_request::HttpRequest;


pub struct Route {
    pub regex_path: String,
    pub path: String,
    pub callback: Box<dyn FnMut() -> HttpResult + 'static>
}

pub struct HttpServer {
    pub get_routes: Vec<Route>
}

pub struct Request {
    method: http::Method,
    params: HashMap<String, String>,
    query :HashMap<String, String>,
}


impl HttpServer {
    pub fn new() -> Self {
        Self {
            get_routes: Vec::new()
        }
    }

    pub fn get(&mut self, path: &str, callback: Box<dyn FnMut() -> HttpResult>) {
        let match_url_param_pattern = r":(\w+)";

        let match_url_param_regex = Regex::new(match_url_param_pattern).unwrap();

        // /users/(\d+)/books/(\d+)
        let key: std::borrow::Cow<'_, str> = match_url_param_regex.replace_all(path, r"([^/]+)");

        let key_regex = Regex::new(&key).unwrap();

        self.get_routes.push(Route{
            path: String::from(path),
            regex_path: String::from(key).add("$"),
            callback
        });
    }

    pub fn match_route(&self, resource: HttpInputStream) -> Option<&Route> {
        for route in &self.get_routes {
            let regex = Regex::new(&route.regex_path).unwrap();

            // dbg!(String::from(r"(.+)/"));
            // dbg!(&route.regex_path);
            // dbg!(&regex.to_string());
            // dbg!(regex.is_match(&resource.relative_path));
            // println!("------------------");

            if regex.is_match(&resource.relative_path) {
                return Some(route)
            }
        }

        return None;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{HttpServer, http_input_stream::HttpInputStream};

    #[test]
    fn it_parse_params_request(){
        let mut request = HttpServer::new();

        request.get("/users", Box::new(||{ crate::HttpResult::new() }));
        request.get("/users/:userId", Box::new(||{ crate::HttpResult::new() }));
        request.get("/users/:userId/books", Box::new(||{ crate::HttpResult::new() }));
        request.get("/users/:userId/books/:bookId", Box::new(||{ crate::HttpResult::new() }));

        let input = HttpInputStream::parse("GET /users HTTP/1.1");
        let route_matched = request.match_route(input);
        assert!(route_matched.is_some());
        assert_eq!(route_matched.unwrap().path, "/users");

        let input = HttpInputStream::parse("GET /users/10 HTTP/1.1");
        let route_matched = request.match_route(input);
        assert!(route_matched.is_some());
        assert_eq!(route_matched.unwrap().path, "/users/:userId");

        let input = HttpInputStream::parse("GET /users/10/books HTTP/1.1");
        let route_matched = request.match_route(input);
        assert!(route_matched.is_some());
        assert_eq!(route_matched.unwrap().path, "/users/:userId/books");


        // let input = HttpInputStream::parse("GET /users/10/books/123 HTTP/1.1");
        // let route_matched = request.match_route(input);
        // assert!(route_matched.is_some());

        // let input = HttpInputStream::parse("GET /books HTTP/1.1");
        // let route_matched = request.match_route(input);
        // assert!(route_matched.is_none());

        // let input = HttpInputStream::parse("GET /users/10/books/123/chapters HTTP/1.1");
        // let route_matched = request.match_route(input);

        // dbg!(&route_matched.unwrap().regex_path);
        // assert!(route_matched.is_none());

    }
}