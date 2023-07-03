/*
 * Copyright 2023 DataOpen SDK Authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::collections::HashMap;
use std::fmt;

impl fmt::Display for ParamsValueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParamsValueType::String(s) => write!(f, "{}", s),
            ParamsValueType::Integer(i) => write!(f, "{}", i),
            ParamsValueType::Float(fl) => write!(f, "{}", fl),
            ParamsValueType::Boolean(b) => write!(f, "{}", b),
        }
    }
}

pub enum ParamsValueType {
    String(String),
    Integer(i32),
    Float(f32),
    Boolean(bool),
}

mod client {
    use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
    use serde_json::value::Value;
    use std::collections::HashMap;
    use std::time::{SystemTime, UNIX_EPOCH};

    use serde::{Deserialize, Serialize};

    use super::ParamsValueType;

    #[derive(Debug, Serialize, Deserialize)]
    struct ApiResponse {
        code: i32,
        message: String,
        data: Option<Data>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Data {
        access_token: String,
        ttl: i32,
    }

    pub struct Client<'a> {
        pub app_id: &'a str,
        pub app_secret: &'a str,
        pub url: Option<&'a str>,
        // dataopen_staging, dataopen
        pub env: Option<&'a str>,
        pub expiration: Option<&'a str>,
        _access_token: String,
        // 单位s
        _ttl: i32,
        _token_time: i32,
    }

    static METHOD_ALLOWED: [&'static str; 5] = ["POST", "GET", "DELETE", "PUT", "PATCH"];

    static OPEN_APIS_PATH: &'static str = "/open-apis";

    impl Client<'_> {
        pub fn new<'a>(
            app_id: &'a str,
            app_secret: &'a str,
            url: Option<&'a str>,
            env: Option<&'a str>,
            expiration: Option<&'a str>,
        ) -> Client<'a> {
            let _url = match url {
                Some(u) if !u.is_empty() => u,
                _ => "https://analytics.volcengineapi.com",
            };

            let _env = match env {
                Some(u) if !u.is_empty() => u,
                _ => "dataopen",
            };

            let _expiration = match expiration {
                Some(u) if !u.is_empty() => u,
                _ => "1800",
            };

            Client {
                app_id,
                app_secret,
                url: Some(_url),
                env: Some(_env),
                expiration: Some(_expiration),
                _ttl: 0,
                _access_token: "".to_owned(),
                _token_time: 0,
            }
        }

        pub async fn request(
            &mut self,
            service_url: &str,
            method: &str,
            headers: HashMap<String, String>,
            params: HashMap<String, ParamsValueType>,
            body: HashMap<String, Value>,
        ) -> Result<HashMap<String, Value>, reqwest::Error> {
            let upper_case_method = method.to_uppercase();

            // if !METHOD_ALLOWED.contains(&upper_case_method.as_str()) {
            // }

            let mut new_headers = HeaderMap::new();

            println!("self._access_token: {}", self._access_token);

            if self._access_token.is_empty()
                || self._access_token.len() == 0
                || !self._valid_token()
            {
                self.get_token().await;
            }

            new_headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&self._access_token).unwrap(),
            );
            new_headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_str("application/json").unwrap(),
            );

            for (key, value) in headers.clone() {
                new_headers.insert(
                    HeaderName::from_bytes(key.as_bytes()).unwrap(),
                    HeaderValue::from_str(&value).unwrap(),
                );
            }

            let completed_url: String = self.url.unwrap_or_default().to_string()
                + "/"
                + self.env.unwrap()
                + OPEN_APIS_PATH
                + &service_url;

            return self
                ._request(
                    &completed_url,
                    &upper_case_method,
                    new_headers.clone(),
                    params,
                    body,
                )
                .await;
        }

        pub(crate) async fn _request(
            &self,
            completed_url: &str,
            method: &str,
            headers: HeaderMap,
            params: HashMap<String, ParamsValueType>,
            body: HashMap<String, Value>,
        ) -> Result<HashMap<String, Value>, reqwest::Error> {
            let client = reqwest::Client::new();
            let query_url = self._joint_query(completed_url, params);

            let mut resp = HashMap::new();

            println!("query_url: {}", query_url);
            println!("headers: {:#?}", headers);
            println!("method: {}", method);
            println!("body: {:#?}", body);

            if method == "GET" {
                resp = client
                    .get(query_url)
                    .headers(headers)
                    .send()
                    .await?
                    .json()
                    .await?;
            } else if method == "POST" {
                resp = client
                    .post(query_url)
                    .headers(headers)
                    .json(&body)
                    .send()
                    .await?
                    .json()
                    .await?;
            } else if method == "PUT" {
                resp = client
                    .put(query_url)
                    .headers(headers)
                    .json(&body)
                    .send()
                    .await?
                    .json()
                    .await?;
            } else if method == "DELETE" {
                resp = client
                    .delete(query_url)
                    .headers(headers)
                    .json(&body)
                    .send()
                    .await?
                    .json()
                    .await?;
            } else if method == "PATCH" {
                resp = client
                    .patch(query_url)
                    .headers(headers)
                    .json(&body)
                    .send()
                    .await?
                    .json()
                    .await?;
            }

            println!("resp: {:#?}", resp);

            Ok(resp)
        }

        pub(crate) async fn get_token(&mut self) {
            let authorization_url: String =
                self.env.unwrap().to_string() + OPEN_APIS_PATH + "/v1/authorization";

            let completed_url: String =
                self.url.unwrap_or_default().to_string() + "/" + &authorization_url;

            let client = reqwest::Client::new();

            let mut map = HashMap::new();
            map.insert("app_id", self.app_id);
            map.insert("app_secret", self.app_secret);

            let resp = client
                .post(completed_url)
                .json(&map)
                .send()
                .await
                .unwrap()
                .json::<ApiResponse>()
                .await
                .unwrap();

            println!("{:#?}", resp);

            let token_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i32;

            // 将resp数据放入
            if resp.code == 200 {
                if let Some(data) = resp.data {
                    self._ttl = data.ttl;
                    self._token_time = token_time;
                    self._access_token = data.access_token;
                }
            }
        }

        pub fn is_authenticated(self) -> bool {
            !self._access_token.is_empty()
        }

        // 拼接 url + query参数，返回url。
        // https://example.com/api
        // { key1: xxx, key2: yyy}
        // => https://example.com/api?key1=xxx&key2=yyy
        pub(crate) fn _joint_query(
            &self,
            url: &str,
            params: HashMap<String, ParamsValueType>,
        ) -> String {
            let mut query_url = url.to_string();

            for (key, value) in &params {
                if query_url.contains("?") {
                    query_url.push_str(&format!("&{}={}", key, value));
                } else {
                    query_url.push_str(&format!("?{}={}", key, value));
                }
            }

            query_url
        }

        // 比较当前token时间是否过期
        pub(crate) fn _valid_token(&self) -> bool {
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i32;

            println!("current time: {}", current_time);
            println!("token time: {}", self._token_time);

            if current_time - self._token_time > self._ttl * 1000 {
                return false;
            }

            return true;
        }
    }
}

// 测试鉴权接口
#[tokio::test]
async fn test_access_token() {
    let app_id = "";
    let app_secret = "";

    let mut client = client::Client::new(app_id, app_secret, None, None, None);

    client.get_token().await;

    println!("输出结果：\\n{:#?}", client.is_authenticated());
}

// 获取实验列表，GET
#[tokio::test]
async fn test_request_get() {
    let app_id = "";
    let app_secret = "";

    let mut client = client::Client::new(app_id, app_secret, None, None, None);

    let map = HashMap::new();

    let mut params = HashMap::new();

    let body = HashMap::new();

    params.insert("app".to_string(), ParamsValueType::Integer(46));
    params.insert("page_size".to_string(), ParamsValueType::Integer(2));
    params.insert("page".to_string(), ParamsValueType::Integer(1));

    let c = client
        .request(
            "/libra/openapi/v1/open/flight-list",
            "GET",
            map,
            params,
            body,
        )
        .await;

    println!("输出结果：\\n{:#?}", c);
}

use serde_json::value::to_value;
use serde_json::{json, Value};

// 新建测试白名单列表，POST
#[tokio::test]
async fn test_request_post() {
    let app_id = "";
    let app_secret = "";

    let mut client = client::Client::new(app_id, app_secret, None, None, None);

    let map = HashMap::new();

    let params = HashMap::new();

    let mut body = HashMap::new();
    body.insert(
        "uid_list".to_string(),
        to_value(&["1111111110000"]).unwrap(),
    );

    let c = client
        .request(
            "/libra/openapi/v1/open/flight/version/6290880/add-test-user",
            "POST",
            map,
            params,
            body,
        )
        .await;

    println!("输出结果：\\n{:#?}", c);
}

#[tokio::test]
async fn test_material_request_post() {
    let app_id = "";
    let app_secret = "";

    let mut client = client::Client::new(
        app_id,
        app_secret,
        Some("https://analytics.volcengineapi.com"),
        Some("dataopen_staging"),
        None,
    );

    let map = HashMap::new();

    let params = HashMap::new();

    let mut body = HashMap::new();
    body.insert("name".to_string(), to_value(&"ccnnodetest").unwrap());
    body.insert("title".to_string(), to_value(&"测试title").unwrap());
    body.insert("type".to_string(), to_value(&"component").unwrap());
    body.insert(
        "description".to_string(),
        to_value(&"测试description5").unwrap(),
    );
    body.insert("frameworkType".to_string(), to_value(&"react").unwrap());

    let c = client
        .request("/material/openapi/v1/material", "PUT", map, params, body)
        .await;

    println!("输出结果：\\n{:#?}", c);
}
