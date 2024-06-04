use axum::{
    body::Body,
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::fmt::Debug;

pub const MSG_SUCCESS: &str = "success";
pub const CODE_SUCCESS: i32 = 0;
pub const CODE_FAIL_BAD_REQEUST: i32 = -1;
pub const CODE_FAIL_INTERNAL: i32 = -2;

#[derive(Debug, Serialize, Default)]
pub struct Res<T> {
    pub code: Option<i32>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Default)]
pub struct Empty {}

#[derive(Debug, Clone)]
pub struct ResJsonString(pub String);

impl<T> IntoResponse for Res<T>
where
    T: Serialize + Send + Sync + Debug + 'static,
{
    fn into_response(self) -> Response {
        let data = Self {
            code: self.code,
            msg: self.msg,
            data: self.data,
        };
        let json_string = match serde_json::to_string(&data) {
            Ok(v) => v,
            Err(e) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                    )
                    .body(Body::from(e.to_string()))
                    .unwrap()
            }
        };
        let res_json_string = ResJsonString(json_string.clone());
        let mut response = json_string.into_response();
        response.extensions_mut().insert(res_json_string);
        response
    }
}

impl<T: Serialize> Res<T> {
    pub fn with_msg(msg: &str) -> Self {
        Self {
            code: Some(CODE_SUCCESS),
            data: None,
            msg: Some(msg.to_string()),
        }
    }

    pub fn with_err(code: i32, msg: &str) -> Self {
        Self {
            code: Some(code),
            msg: Some(msg.to_string()),
            data: None,
        }
    }

    pub fn with_data(data: T) -> Self {
        Self {
            code: Some(CODE_SUCCESS),
            msg: Some(MSG_SUCCESS.to_string()),
            data: Some(data),
        }
    }
    #[allow(dead_code)]
    pub fn with_data_msg(data: T, msg: &str) -> Self {
        Self {
            code: Some(200),
            data: Some(data),
            msg: Some(msg.to_string()),
        }
    }
}
