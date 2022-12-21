use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
}

impl FromStr for Method {

    type Err = ();

    fn from_str(input: &str) -> Result<Method, Self::Err> {
        match input.to_uppercase().borrow() {
            "GET" => Ok(Method::GET),
            "HEAD" => Ok(Method::HEAD),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            _ => Err(()),
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self).unwrap();
        Ok(())
    }
}
