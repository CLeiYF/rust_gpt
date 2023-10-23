use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use ring::hmac;
use serde_json::Value;
use url::Url;

//星火大模型V2请求地址，对应的domain参数为generalv2
const HOST_URL: &str = "https://spark-api.xf-yun.com/v1.1/chat";
// 用于配置大模型版本，默认“general/generalv2”
const DOMAIN: &str = "general"; // v1.5版本

pub struct BigMod {
    app_id: String,
    api_secret: String,
    app_key: String,
}

impl BigMod {
    pub fn new(app_id: String, api_secret: String, app_key: String) -> Self {
        BigMod {
            app_id,
            api_secret,
            app_key,
        }
    }

    pub fn get_auth_url(&self) -> String {
        let mut auth_url = Url::parse(HOST_URL).unwrap();
        let url = auth_url.clone();

        // 将日期时间格式化为RFC1123格式
        let date = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
        // 拼接
        let host = url.host_str().unwrap();
        let prestr = format!(
            "host: {}\ndate: {}\nGET {} HTTP/1.1",
            host,
            date,
            url.path()
        );

        // 利用hmac-sha256算法结合APISecret对上一步的tmp签名，获得签名后的摘要tmp_sha
        let key = hmac::Key::new(hmac::HMAC_SHA256, self.api_secret.as_bytes());
        // 将上方的tmp_sha进行base64编码生成signature
        let signature = general_purpose::STANDARD.encode(hmac::sign(&key, prestr.as_bytes()));
        // 利用上面生成的signature，拼接下方的字符串生成authorization_origin
        let authorization = format!(
            "api_key=\"{}\", algorithm=\"{}\", headers=\"{}\", signature=\"{}\"",
            self.app_key, "hmac-sha256", "host date request-line", signature,
        );

        auth_url
            .query_pairs_mut()
            .append_pair(
                "authorization",
                &general_purpose::STANDARD.encode(authorization.as_bytes()),
            )
            .append_pair("date", &date)
            .append_pair("host", host);

        auth_url.to_string()
    }

    pub fn gen_params(&self, question: String) -> String {
        //通过appid和用户的提问来生成请参数
        let json = r#"
            {
            "header": {
                "app_id": "",
                "uid": "1234"
            },
            "parameter": {
                "chat": {
                    "domain": "",
                    "temperature": 0.5,
                    "max_tokens": 2048
                }
            },
            "payload": {
                "message": {
                    "text": ""
                }
            }
        }
        "#;

        let mut data: Value = serde_json::from_str(json).expect("Parse faild");
        data["header"]["app_id"] = Value::String(self.app_id.clone());
        data["parameter"]["chat"]["domain"] = Value::String(String::from(DOMAIN));
        data["payload"]["message"]["text"] = Value::Array(get_text(String::from("user"), question));

        return String::from(serde_json::to_string(&data).expect("Parse filed"));
    }
}

fn get_text(role: String, content: String) -> Vec<Value> {
    let mut json: Value = serde_json::from_str("{}").unwrap();
    json["role"] = Value::String(role);
    json["content"] = Value::String(content);

    vec![json]
}
