use std::collections::HashMap;
use regex::Regex;
use url::{Url, ParseError};
use std::hash::{Hash, Hasher};

struct RegexWrapper(Regex);

impl PartialEq for RegexWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_str() == other.0.as_str()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for RegexWrapper{}

impl Hash for RegexWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_str().hash(state);
    }
}

#[derive(PartialEq, Debug)]
enum HttpVerbs {
    GET,
    POST,
    PUT,
    DELETE
} 

pub struct HttpServer<'a> {
    pub get_routes: HashMap<RegexWrapper, Box<dyn FnMut() + 'a>>,
}

pub struct teste<'a> {
    key: &'a str,
    value: &'a Box<dyn FnMut() + 'a>
}


pub struct Request<'a> {
    method: HttpVerbs,
    original_url: &'a str,
    params: HashMap<&'a str, &'a str>,
    query :HashMap<String, String>,
}

#[derive(Debug)]
pub enum Errors  {
   HttpVerbNotImplemented 
}  

impl<'a> HttpServer<'a> {
    pub fn new() -> Self {
        Self {
            get_routes: HashMap::new()
        }
    }

    pub fn get(&mut self, path: &'a str, callback: Box<dyn FnMut()>) {
        let pattern = r":(\w+)";
        let regex = Regex::new(pattern).expect("1º -> Wrong regex");

        let key = regex.replace_all(path, r"(\w+)");

        let key_regex = Regex::new(&key).expect("2º -> Wrong regex");

        self.get_routes.insert(RegexWrapper(key_regex), callback);
    }

    pub fn match_route(&self, resource: &'a str) -> Option<teste> {
        let binding = resource.split(" ").collect::<Vec<&str>>();
        
        let original_url = binding[1];

        for (key, value) in &self.get_routes {
            if key.0.is_match(original_url) {
                return Some(teste {
                    key: key.0.as_str(),
                    value: value
                })
            } 
        }

        return None;
    }

    pub fn parse(&self, stream_str: &'a str, path_matched: &str) -> Result<Request, Errors> {
        let binding = stream_str.split(" ").collect::<Vec<&str>>();
        
        let original_url = binding[1];

        // let url = Url::parse(original_url).unwrap();

        // let query = url.query_pairs().into_owned().collect::<HashMap<String,String>>();

        let mut params: HashMap<&str,&str> = HashMap::new();

        let regex = Regex::new(path_matched).expect("Erro ao compilar a regex");

        // Verifique se a regex corresponde à entrada
        if let Some(captures) = regex.captures(original_url) {
            // Captura os valores correspondentes aos grupos na regex
            let a = original_url.split("/").into_iter().enumerate();

            let mut _index = 0;
            for (index, url_part) in a {
                println!("{}", path_matched);

                if url_part.starts_with(":") {
                    if let Some(value) = captures.get(_index) {
                        println!("{} {}", url_part, value.as_str());
                        params.insert(url_part, value.as_str());
                    }

                    _index += 1;
                }
            }
        } else {
            println!("A entrada não corresponde à regex.");
        }

        if stream_str.starts_with("GET") {
            return Ok(Request {
                method: HttpVerbs::GET,
                original_url,
                params,
                query: HashMap::new()
            })
        }

        // if resource.starts_with("POST") {
        //     return Ok(Request {
        //         method: HttpVerbs::POST,
        //         original_url: "",
        //         params: HashMap::new(),
        //         query: HashMap::new(),
        //     })
        // }

        // if resource.starts_with("PUT") {
        //     return Ok(Request {
        //         method: HttpVerbs::PUT,
        //         original_url: "",
        //         params: HashMap::new(),
        //         query: HashMap::new(),
        //     })
        // }

        // if resource.starts_with("DELETE") {
        //     return Ok(Request {
        //         method: HttpVerbs::DELETE,
        //         original_url: "",
        //         params: HashMap::new(),
        //         query: HashMap::new(),
        //     })
        // }

        Err(Errors::HttpVerbNotImplemented)
    }
}

#[cfg(test)]
pub mod tests {

    use crate::request::{HttpServer, HttpVerbs};


    #[test]
    fn it_parse_params_request(){
        let mut request = HttpServer::new();

        request.get("/users/:userId/books/:bookId", Box::new(||{}));

        let resouce = "GET /users/10/books/123 HTTP/1.1";

        let res = request.match_route(resouce);

        match res {
            Some(teste) => {

                let result = request.parse("GET /users/153/books/123 HTTP/1.1", teste.key).unwrap();
                assert_eq!(result.method, HttpVerbs::GET);

                // let author_name= result.params.get("userId");
                // let book_name= result.params.get("bookId");
        
                // assert_eq!(*author_name.unwrap(), "153");
                // assert_eq!(*book_name.unwrap(), "123");
            },
            None => {}
        }
       
    }

}