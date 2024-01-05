use std::{collections::HashMap, net::{TcpListener, SocketAddr, Ipv4Addr, IpAddr}};
mod request;

pub struct Server<'a> {
    pub is_running: bool,
    pub get_routes: HashMap<&'a str, Box<dyn FnMut() + 'a>>
}

impl<'a> Server<'a> {
    pub fn new() -> Self {
        
        Self {
            is_running: false,
            get_routes: HashMap::new()
        }
    }

    pub fn get(&mut self, path: &'static str, callback: Box<dyn FnMut()>){
        self.get_routes.insert(path, callback);
    }

    pub fn bind(&mut self, port: u16){
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 100, 43)), port);

        let listener = match TcpListener::bind(addr) {
            Ok(listener) => {
                self.is_running = true;
                println!("Servidor rodando: http://{}", addr);
                listener
            },
            Err(err) => {
                panic!("Nao foi possível subir o servidor: {}", err)
            }
        };

        for stream in listener.incoming() {

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use request::tests;

    // use std::sync::{Barrier, Arc};



    // #[test]
    // fn it_works() {
    //     let server = Server::new();
    //     assert!(server.get_routes.is_empty())
    // }

    // #[test]
    // fn it_push_one_get_route(){
    //     let mut server = Server::new();

    //     server.get("/", Box::new(|| {

    //     }));

    //     assert!(server.get_routes.len() > 0)
    // }

    // #[test]
    // fn it_bind_works(){
    //     let mut server = Server::new();

    //     server.bind(8080);

    //     assert!(server.is_running)
    // }

    // #[test]
    // fn it_response_get_route(){
    //     let mut server = Server::new();


    //     let barrier = Arc::new(Barrier::new(2));
    //     let barrier_clone = barrier.clone();


    //     let a = move || {
    //         assert!(true);
    //         barrier_clone.wait();
    //     };

    //     server.get("/", Box::new(a));


    //     barrier.wait();
    // }
}
