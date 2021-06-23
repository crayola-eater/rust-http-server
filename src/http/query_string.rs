use std::collections::HashMap;

pub struct QueryString<'buffer> {
    data: HashMap<&'buffer str, QueryParameterValue<'buffer>>,
}

pub enum QueryParameterValue<'buffer> {
    Single(&'buffer str),
    Multiple(Vec<&'buffer str>),
}

impl<'buffer> QueryString<'buffer> {
    pub fn get(&self, key: &str) -> Option<&QueryParameterValue> {
        self.data.get(key)
    }
}
