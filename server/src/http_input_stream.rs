pub struct HttpInputStream<'a> {
    pub method: &'a str,
    pub relative_path: &'a str,
    pub http_version: &'a str
}


impl<'a> HttpInputStream<'a> {
    pub fn default() -> Self {
        Self {
           http_version: "",
           method: "",
           relative_path: ""
        }
    }

    // GET /users/.... HTTP/1.1
    pub fn parse(input_stream: &'a str) -> Self {
        let resouce_parts = input_stream.split(" ").collect::<Vec<&str>>();

        Self {
            method: resouce_parts[0],
            relative_path: resouce_parts[1],
            http_version: resouce_parts[2],
        }
    }
}


#[cfg(test)]
pub mod test {
    use super::HttpInputStream;

    #[test]
    pub fn it_can_create(){
        let input_stream_parse = HttpInputStream::default();

        assert!(input_stream_parse.method.is_empty());
        assert!(input_stream_parse.relative_path.is_empty());
        assert!(input_stream_parse.http_version.is_empty());
    }

    #[test]
    pub fn it_can_parse_input_strean(){

        let input_stream = "GET /users HTTP/1.1";

        let input_stream_parse = HttpInputStream::parse(input_stream);
        
        assert_eq!(input_stream_parse.method, "GET");
        assert_eq!(input_stream_parse.relative_path, "/users");
        assert_eq!(input_stream_parse.http_version, "HTTP/1.1");
    }
}