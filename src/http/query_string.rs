use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, k: &str) -> Option<&Value> {
        self.data.get(k)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split('&') {
            let mut k = sub_str;
            let mut v = "";
            if let Some(i) = sub_str.find("=") {
                k = &sub_str[..i];
                v = &sub_str[i + 1..];
            }
            data.entry(k)
                .and_modify(|cur: &mut Value| match cur {
                    Value::Single(prev) => *cur = Value::Multiple(vec![prev, v]),
                    Value::Multiple(vec) => vec.push(v),
                })
                .or_insert(Value::Single(v));
        }
        QueryString { data }
    }
}
