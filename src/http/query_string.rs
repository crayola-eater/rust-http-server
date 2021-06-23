use std::collections::HashMap;

pub struct QueryString<'buffer> {
    data: HashMap<&'buffer str, QueryParameterValue<'buffer>>,
}

pub enum QueryParameterValue<'buffer> {
    Single(&'buffer str),
    Multiple(Vec<&'buffer str>),
}
