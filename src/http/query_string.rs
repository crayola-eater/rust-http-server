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

impl<'buffer> From<&'buffer str> for QueryString<'buffer> {
    fn from(raw: &'buffer str) -> Self {
        // a=1&b=2&c=3&a=4
        let data = raw.split('&').fold(HashMap::new(), |mut map, pair| {
            let mut iterator = pair.split('=');

            match (iterator.next(), iterator.next()) {
                (Some(key), Some(value)) => {
                    map.entry(key)
                        .and_modify(|existing| match existing {
                            QueryParameterValue::Single(previous) => {
                                *existing = QueryParameterValue::Multiple(vec![previous, value])
                            }
                            QueryParameterValue::Multiple(vec) => vec.push(value),
                        })
                        .or_insert(QueryParameterValue::Single(value));
                }
                _ => println!("Failed to parse key-value pair: '{}'", pair),
            };

            map
        });

        Self { data }
    }
}
