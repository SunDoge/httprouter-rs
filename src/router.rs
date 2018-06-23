// use http::{Request, Response};
use hyper::{Body, Request, Response};

pub type Handle<T> = fn(Request<T>) -> Response<T>;

#[derive(Debug, Clone)]
pub struct Param {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub struct Params(pub Vec<Param>);

impl Params {
    pub fn by_name(&self, name: &str) -> Option<String> {
        match self.0.iter().find(|param| param.key == name) {
            Some(param) => Some(param.value.clone()),
            None => None,
        }
    }
}

pub struct Router {}

impl Router {
    pub fn new() -> Router {
        Router {}
    }

    pub fn get() {}

    pub fn post() {}

    pub fn put() {}

    pub fn patch() {}

    pub fn delete() {}

    pub fn group() {}

    pub fn serve_files(&mut self, path: &str) {
        if path.as_bytes().len() < 10 || &path[path.len() - 10..] != "/*filepath" {
            panic!("path must end with /*filepath in path '{}'", path);
        }
    }

    pub fn handle<T>(&mut self, method: &str, path: &str, handle: Handle<T>) {
        if !path.starts_with("/") {
            panic!("path must begin with '/' in path '{}'", path);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn params() {
        use router::{Param, Params};

        let params = Params(vec![
            Param {
                key: "fuck".to_owned(),
                value: "you".to_owned(),
            },
            Param {
                key: "lalala".to_string(),
                value: "papapa".to_string(),
            },
        ]);

        assert_eq!(Some(String::from("you")), params.by_name("fuck"));
        assert_eq!(Some(String::from("papapa")), params.by_name("lalala"));
    }

    #[test]
    #[should_panic(expected = "path must begin with '/' in path 'something'")]
    fn handle_ivalid_path() {
        use http::Response;
        use router::Router;

        let path = "something";
        let mut router = Router::new();

        router.handle("GET", path, |_req| Response::new(()));
    }
}
