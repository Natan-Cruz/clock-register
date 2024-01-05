use std::collections::HashMap;
use url::{Url, ParseError};


#[derive(PartialEq, Debug)]
enum HttpVerbs {
    GET,
    POST,
    PUT,
    DELETE
} 

#[derive(PartialEq, Debug)]
pub struct Request<'a> {
    method: HttpVerbs,
    original_url: &'a str,
    params:HashMap<String, String>,
    query :HashMap<String, String>,
}


#[derive(PartialEq, Debug)]
pub enum Errors  {
   HttpVerbNotImplemented 
}  

impl<'a> Request<'a> {
    pub fn parse(resource: &'a str) -> Result<Self, Errors> {
        let binding = resource.split(" ").collect::<Vec<&str>>();
        
        let original_url = binding[1];

        let url = Url::parse(original_url).unwrap();

        let params = url.pa().into_owned().collect::<HashMap<String,String>>();

        let query = url.query_pairs().into_owned().collect::<HashMap<String,String>>();


        if resource.starts_with("GET") {
            return Ok(Request {
                method: HttpVerbs::GET,
                original_url,
                params: HashMap::new(),
                query
            })
        }

        if resource.starts_with("POST") {
            return Ok(Request {
                method: HttpVerbs::POST,
                original_url: "",
                params: HashMap::new(),
                query: HashMap::new(),
            })
        }

        if resource.starts_with("PUT") {
            return Ok(Request {
                method: HttpVerbs::PUT,
                original_url: "",
                params: HashMap::new(),
                query: HashMap::new(),
            })
        }

        if resource.starts_with("DELETE") {
            return Ok(Request {
                method: HttpVerbs::DELETE,
                original_url: "",
                params: HashMap::new(),
                query: HashMap::new(),
            })
        }

        Err(Errors::HttpVerbNotImplemented)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::request::{Request, HttpVerbs, Errors};

    // #[test]
    // fn it_parse_not_implement_http_verb_request(){
    //     let request = Request::parse("CHECK / HTTP/1.1")
    //         .expect_err("Erro esperado pois não há nenhum http verbo chamado CHECK");

    //     assert_eq!(request, Errors::HttpVerbNotImplemented)
    // }

    // #[test]
    // fn it_parse_get_request(){
    //     let request = Request::parse("GET http://localhost:8000/ HTTP/1.1").unwrap();

    //     assert_eq!(request.method, HttpVerbs::GET)
    // }

    // #[test]
    // fn it_parse_post_request(){
    //     let request = Request::parse("POST http://localhost:8000/ HTTP/1.1").unwrap();

    //     assert_eq!(request.method, HttpVerbs::POST)
    // }

    // #[test]
    // fn it_parse_put_request(){
    //     let request = Request::parse("PUT http://localhost:8000/ HTTP/1.1").unwrap();

    //     assert_eq!(request.method, HttpVerbs::PUT)
    // }

    // #[test]
    // fn it_parse_delete_request(){
    //     let request = Request::parse("DELETE http://localhost:8000/ HTTP/1.1").unwrap();

    //     assert_eq!(request.method, HttpVerbs::DELETE)
    // }

    // #[test]
    // fn it_parse_original_url_prop_request(){
    //     let request = Request::parse("GET http://localhost:8000/teste HTTP/1.1").unwrap();

    //     assert_eq!(request.original_url, "/teste")
    // }

    #[test]
    fn it_parse_params_request(){
        let request = Request::parse("GET /authors/Tio-Bob/books/Clear-Code HTTP/1.1").unwrap();

        
        let author_name= request.params.get("author_name");
        let book_name= request.params.get("book_name");

        assert_eq!(author_name.unwrap(), "Tio-Bob");
        assert_eq!(book_name.unwrap(), "Clear-Code");
    }

    #[test]
    fn it_parse_query_request(){
        let request = Request::parse("GET http://localhost:8000/cars?type=hatch&color=red HTTP/1.1").unwrap();
        
        let car_type= request.query.get("type");
        let car_color= request.query.get("color");

        assert_eq!(car_type.unwrap(), "hatch");
        assert_eq!(car_color.unwrap(), "red");
    }
}