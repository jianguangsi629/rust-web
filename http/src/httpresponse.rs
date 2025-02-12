use std::collections::HashMap;
use std::io::{Result, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<&'a str>,
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse<'a>) -> String {
       let res1 = res.clone();
       format!("{} {} {}\r\nContent-Length: {}\r\n\r\n{}", 
       &res1.version(), 
       &res1.status_code(),
       &res1.status_text(),
       &res.body.unwrap().len(),
       &res.body.unwrap())
    }
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        // 默认的HTTP响应 返回结构体
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str, 
        headers: Option<HashMap<&'a str, &'a str>>, 
        body: Option<&'a str>) 
    -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200"{
            response.status_code = status_code;
        }
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };

        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "404" => "NOT FOUND".into(),
            _ => "NOT FOUND".into(),
        };

        response.body = body;
        response
    }


    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        let response_string: String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);
        
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
        let mut header_string: String = "".to_string();
        for (k, v) in map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b,
            None => "",
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new("200", None, Some("xxx").into());
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxx").into(),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_response_struct_creation_404() {
        let response_actual = HttpResponse::new("404", None, Some("xxx").into());
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxx").into(),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_response_struct_creation() {
        let response_expected = HttpResponse{
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: None,
            body: None,
        };
        let http_string: String = response_expected.into();
        let actual_string = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n".to_string();
        assert_eq!(http_string, actual_string);
    }
}