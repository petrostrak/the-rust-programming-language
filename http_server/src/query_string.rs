use std::collections::HashMap;

pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(value: &'buf str) -> Self {
        let mut data: HashMap<&str, Value> = HashMap::new();
        for sub_str in value.split('&') {
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|ex_val| match ex_val {
                    Value::Single(prev_val) => {
                        *ex_val = Value::Multiple(vec![prev_val, val]);
                    }
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
        }
        unimplemented!()
    }
}
