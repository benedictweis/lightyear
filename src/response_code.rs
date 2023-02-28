#[derive(Copy, Clone)]
pub enum ResponseCode {
    Continue = 100,
    OK = 200,
    MovedPermanently = 301,
    Found = 302,
    NotFound = 404,
    MethodNotAllowed = 405,
}

impl TryFrom<usize> for ResponseCode {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            x if x == ResponseCode::Continue as usize => Ok(ResponseCode::Continue),
            x if x == ResponseCode::OK as usize => Ok(ResponseCode::OK),
            x if x == ResponseCode::MovedPermanently as usize => Ok(ResponseCode::MovedPermanently),
            x if x == ResponseCode::Found as usize => Ok(ResponseCode::Found),
            x if x == ResponseCode::NotFound as usize => Ok(ResponseCode::NotFound),
            x if x == ResponseCode::MethodNotAllowed as usize => Ok(ResponseCode::MethodNotAllowed),
            _ => Err("response code not found".into()),
        }
    }
}

impl ResponseCode {
    pub fn compose(self) -> String {
        match self {
            ResponseCode::Continue => format!("{} Continue", self as usize),
            ResponseCode::OK => format!("{} OK", self as usize),
            ResponseCode::MovedPermanently => format!("{} Moved Permanently", self as usize),
            ResponseCode::Found => format!("{} Found", self as usize),
            ResponseCode::NotFound => format!("{} Not Found", self as usize),
            ResponseCode::MethodNotAllowed => format!("{} Method Not Allowed", self as usize),
        }
    }
}
