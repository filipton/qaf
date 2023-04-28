use crate::{request::WasmRequest, response::WasmResponse};
use anyhow::Result;
use matchit::Router;
use std::{collections::HashMap, future::Future, pin::Pin, rc::Rc};

pub type LocalBoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;
pub type AsyncHandlerFn<'a> =
    Rc<dyn 'a + Fn(WasmRequest) -> LocalBoxFuture<'a, Result<WasmResponse>>>;

const METHODS: [&str; 9] = [
    "GET", "POST", "PUT", "DELETE", "HEAD", "OPTIONS", "CONNECT", "PATCH", "TRACE",
];

/// A simple router using matchit.
pub struct WasmRouter<'a> {
    pub routes: HashMap<String, Router<AsyncHandlerFn<'a>>>,
}

impl<'a> WasmRouter<'a> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut routes = HashMap::new();
        for method in METHODS.iter() {
            routes.insert(method.to_string(), Router::new());
        }
        Self { routes }
    }

    #[allow(dead_code)]
    pub fn add_route<T>(&mut self, method: &str, path: &str, handler: fn(WasmRequest) -> T)
    where
        T: Future<Output = Result<WasmResponse>> + 'a,
    {
        let handler: AsyncHandlerFn = Rc::new(move |req: WasmRequest| Box::pin(handler(req)));
        self.routes
            .get_mut(method)
            .unwrap()
            .insert(path, handler)
            .unwrap();
    }

    #[allow(dead_code)]
    pub fn get<T>(mut self, path: &str, handler: fn(WasmRequest) -> T) -> Self
    where
        T: Future<Output = Result<WasmResponse>> + 'a,
    {
        self.add_route("GET", path, handler);
        self
    }

    #[allow(dead_code)]
    pub fn post<T>(mut self, path: &str, handler: fn(WasmRequest) -> T) -> Self
    where
        T: Future<Output = Result<WasmResponse>> + 'a,
    {
        self.add_route("POST", path, handler);
        self
    }

    #[allow(dead_code)]
    pub fn put<T>(mut self, path: &str, handler: fn(WasmRequest) -> T) -> Self
    where
        T: Future<Output = Result<WasmResponse>> + 'a,
    {
        self.add_route("PUT", path, handler);
        self
    }

    #[allow(dead_code)]
    pub fn delete<T>(mut self, path: &str, handler: fn(WasmRequest) -> T) -> Self
    where
        T: Future<Output = Result<WasmResponse>> + 'a,
    {
        self.add_route("DELETE", path, handler);
        self
    }

    #[allow(dead_code)]
    pub fn head<T>(mut self, path: &str, handler: fn(WasmRequest) -> T) -> Self
    where
        T: Future<Output = Result<WasmResponse>> + 'a,
    {
        self.add_route("HEAD", path, handler);
        self
    }

    #[allow(dead_code)]
    pub fn options<T>(mut self, path: &str, handler: fn(WasmRequest) -> T) -> Self
    where
        T: Future<Output = Result<WasmResponse>> + 'a,
    {
        self.add_route("OPTIONS", path, handler);
        self
    }

    #[allow(dead_code)]
    pub fn connect<T>(mut self, path: &str, handler: fn(WasmRequest) -> T) -> Self
    where
        T: Future<Output = Result<WasmResponse>> + 'a,
    {
        self.add_route("CONNECT", path, handler);
        self
    }

    #[allow(dead_code)]
    pub fn patch<T>(mut self, path: &str, handler: fn(WasmRequest) -> T) -> Self
    where
        T: Future<Output = Result<WasmResponse>> + 'a,
    {
        self.add_route("PATCH", path, handler);
        self
    }

    #[allow(dead_code)]
    pub fn trace<T>(mut self, path: &str, handler: fn(WasmRequest) -> T) -> Self
    where
        T: Future<Output = Result<WasmResponse>> + 'a,
    {
        self.add_route("TRACE", path, handler);
        self
    }
}
