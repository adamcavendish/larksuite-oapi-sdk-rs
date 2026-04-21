macro_rules! impl_resp {
    ($name:ident, $data:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub api_resp: $crate::resp::ApiResp,
            pub code_error: $crate::resp::CodeError,
            pub data: Option<$data>,
        }
        impl $name {
            pub fn success(&self) -> bool {
                self.code_error.success()
            }
        }
    };
    ($name:ident) => {
        pub struct $name {
            pub api_resp: $crate::resp::ApiResp,
            pub code_error: Option<$crate::resp::CodeError>,
            pub data: Option<serde_json::Value>,
        }
        impl $name {
            pub fn success(&self) -> bool {
                self.api_resp.status_code == 200
                    && self.code_error.as_ref().is_none_or(|e| e.code == 0)
            }
        }
    };
}

macro_rules! impl_resp_v2 {
    ($name:ident, $data:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub api_resp: $crate::resp::ApiResp,
            pub code_error: Option<$crate::resp::CodeError>,
            pub data: Option<$data>,
        }
        impl $name {
            pub fn success(&self) -> bool {
                self.code_error.as_ref().is_none_or(|e| e.code == 0)
            }
        }
    };
}
