use super::method::HttpMethod;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: HttpMethod,
}
