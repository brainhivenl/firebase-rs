use crate::constants::{
    END_AT, EQUAL_TO, EXPORT, FORMAT, LIMIT_TO_FIRST, LIMIT_TO_LAST, ORDER_BY, SHALLOW, START_AT,
};
use crate::Firebase;
use serde::Serialize;
use url::Url;

#[derive(Debug)]
pub struct Params {
    pub uri: Url,
}

impl Params {
    pub fn new(uri: Url) -> Self {
        Self { uri }
    }

    pub fn add_param<T: Serialize>(&mut self, key: &str, value: T) -> &mut Self {
        self.uri
            .query_pairs_mut()
            .append_pair(key, &serde_json::to_string(&value).unwrap());
        self
    }

    pub fn order_by(&mut self, key: &str) -> &mut Params {
        self.uri
            .query_pairs_mut()
            .append_pair(ORDER_BY, &format!("\"{key}\""));

        self
    }

    pub fn limit_to_first<T: Serialize>(&mut self, count: T) -> &mut Params {
        self.add_param(LIMIT_TO_FIRST, count)
    }

    pub fn limit_to_last<T: Serialize>(&mut self, count: T) -> &mut Params {
        self.add_param(LIMIT_TO_LAST, count)
    }

    pub fn start_at<T: Serialize>(&mut self, index: T) -> &mut Params {
        self.add_param(START_AT, index)
    }

    pub fn end_at<T: Serialize>(&mut self, index: T) -> &mut Params {
        self.add_param(END_AT, index)
    }

    pub fn equal_to<T: Serialize>(&mut self, value: T) -> &mut Params {
        self.add_param(EQUAL_TO, value)
    }

    pub fn shallow(&mut self, flag: bool) -> &mut Params {
        self.add_param(SHALLOW, flag)
    }

    pub fn format(&mut self) -> &mut Params {
        self.uri.query_pairs_mut().append_pair(ORDER_BY, EXPORT);
        self
    }

    pub fn finish(&self) -> Firebase {
        Firebase::new(self.uri.as_str()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::params::Params;
    use url::Url;

    #[test]
    fn check_params() {
        let mut param = Params {
            uri: Url::parse("https://github.com/emreyalvac").unwrap(),
        };

        param.add_param("param_1", "value_1");
        param.add_param("param_2", "value_2");

        assert_eq!(
            param.uri.as_str(),
            "https://github.com/emreyalvac?param_1=%22value_1%22&param_2=%22value_2%22"
        )
    }
}
