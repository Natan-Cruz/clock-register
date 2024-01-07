use std::collections::HashMap;
use regex::Regex;
use url::Url;


#[derive(PartialEq, Debug)]
enum HttpVerbs {
    GET,
    POST,
    PUT,
    DELETE
}

pub struct Route {
    regex_path: String,
    path: String,
    callback: Box<dyn FnMut() + 'static>
}

pub struct HttpServer {
    pub get_routes: Vec<Route>
}

pub struct Request {
    method: HttpVerbs,
    params: HashMap<String, String>,
    query :HashMap<String, String>,
}



impl HttpServer {
    pub fn new() -> Self {
        Self {
            get_routes: Vec::new()
        }
    }

    pub fn get(&mut self, path: &str, callback: Box<dyn FnMut()>) {
        let match_url_param_pattern = r":(\w+)";

        let match_url_param_regex = Regex::new(match_url_param_pattern).unwrap();

        // /users/(\d+)/books/(\d+)
        let key: std::borrow::Cow<'_, str> = match_url_param_regex.replace_all(path, r"(\d+)");

        let key_regex = Regex::new(&key).unwrap();

        self.get_routes.push(Route{
            path: String::from(path),
            regex_path: String::from(key_regex.as_str()),
            callback
        });
    }

    pub fn match_route(&self, resource: &str) -> Option<&Route> {
        let resouce_parts = resource.split(" ").collect::<Vec<&str>>();
        
        let url = resouce_parts[1];

        for route in &self.get_routes {
            let regex = Regex::new(&route.regex_path).unwrap();

            if regex.is_match(&url) {
                return Some(route)
            }
        }

        return None;
    }

    pub fn parse(&self, resource: &str, route: &Route) -> Request {
        let resouce_parts = resource.split(" ").collect::<Vec<&str>>();

        let url = resouce_parts[1];

        let base_url = Url::parse("http://localhost:8000").unwrap();

        let url_parser =  Url::options().base_url(Some(&base_url)).parse(url).unwrap();

        let query = url_parser.query_pairs().into_owned().collect::<HashMap<String,String>>();

        let mut params: HashMap<String, String> = HashMap::new();

        let regex = Regex::new(&route.regex_path).expect("Erro ao compilar a regex");

        // Verifique se a regex corresponde à entrada
        if let Some(captures) = regex.captures(url) {
            // Captura os valores correspondentes aos grupos na regex
            let path_matched_parts = route.path.split("/").collect::<Vec<&str>>();

            let mut _index = 1;

            for url_part in &path_matched_parts {
                if  url_part.starts_with(":") {

                    if let Some(value) = captures.get(_index) {
                        println!("VALUE: {}", &value.as_str());
                        let param_key = &url_part.replace(":", "");

                        params.insert(String::from(param_key), String::from(value.as_str()));
                    }

                    _index += 1;
                }
            }
        } else {
            println!("A entrada não corresponde à regex.");
        }

        return Request {
            method: HttpVerbs::GET,
            params,
            query
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{HttpServer, HttpVerbs};

    #[test]
    fn it_parse_params_request(){
        let mut request = HttpServer::new();

        request.get("/users/:userId/books/:bookId", Box::new(||{}));

        let resouce = "GET /users/10/books/123 HTTP/1.1";

        let res = request.match_route(resouce);

        match res {
            Some(teste) => {

                let result = request.parse("GET /users/153/books/123 HTTP/1.1", teste);
                assert_eq!(result.method, HttpVerbs::GET);

                let author_name= result.params.get("userId");
                let book_name= result.params.get("bookId");
        
                assert_eq!(*author_name.unwrap(), "153");
                assert_eq!(*book_name.unwrap(), "123");
            },
            None => {}
        }
       
    }


     #[test]
    fn it_parse_query_request(){
        let mut request = HttpServer::new();

        request.get("/users", Box::new(||{}));

        let resouce = "GET /users?type=hatch&color=red HTTP/1.1";

        let res = request.match_route(resouce);

        match res {
            Some(teste) => {
                let result = request.parse("GET /users?type=hatch&color=red HTTP/1.1", teste);

                let car_type= result.query.get("type");
                let car_color= result.query.get("color");
        
                assert_eq!(car_type.unwrap(), "hatch");
                assert_eq!(car_color.unwrap(), "red");
            }
            None => {}
        }
    }

}