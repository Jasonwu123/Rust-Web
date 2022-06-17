use std::collections::HashMap;
use std::io::{Result, Write};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(resp: HttpResponse<'a>) -> Self {
        let resp1 = resp.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &resp1.version(),
            &resp1.status_code(),
            &resp1.status_text(),
            &resp1.headers(),
            &resp.body.unwrap().len(),
            &resp1.body(),
        )
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut resp: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
            resp.status_code = status_code.into();
        };

        resp.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };

        resp.status_text = match resp.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not Found".into(),
        };

        resp.body = body;
        resp
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let resp = self.clone();
        let resp = String::from(resp);
        let _ = write!(write_stream, "{}", resp);

        Ok(())
    }

    fn version(&self) -> &str {
        self.version
    }

    fn status_code(&self) -> &str {
        self.status_code
    }

    fn status_text(&self) -> &str {
        self.status_text
    }

    fn headers(&self) -> String {
        let map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string = "".into();

        for (k, v) in map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

/// For test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let resp_actual = HttpResponse::new("200", None, Some("xxxx".into()));
        let resp_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };

        assert_eq!(resp_actual, resp_expected);
    }

    #[test]
    fn test_response_struct_creation_400() {
        let resp_actual = HttpResponse::new("404", None, Some("xxxx".into()));
        let resp_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };

        assert_eq!(resp_actual, resp_expected);
    }
}
