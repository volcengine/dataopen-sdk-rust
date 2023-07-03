fn main() {
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
}