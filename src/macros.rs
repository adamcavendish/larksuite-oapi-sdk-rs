/// Generate [`Client`](crate::Client) service accessor methods.
///
/// Each invocation expands to a `pub fn` that constructs the service version
/// struct and borrows the client's config. Four tokens:
///
/// ```ignore
/// service_accessor!(method_name, module, version, VersionType);
/// service_accessor!(method_name, module, version, VersionType, deprecated = "reason");
/// ```
///
/// `method_name` is the public method name on `Client`. `module`, `version`,
/// and `VersionType` select the path `service::$module::$version::$VersionType`.
macro_rules! service_accessor {
    ($method:ident, $mod:ident, $ver:ident, $vty:ident) => {
        #[inline]
        pub fn $method(&self) -> $crate::service::$mod::$ver::$vty<'_> {
            $crate::service::$mod::$ver::$vty::new(&self.config)
        }
    };
    ($method:ident, $mod:ident, $ver:ident, $vty:ident, deprecated = $note:literal) => {
        #[deprecated(note = $note)]
        #[inline]
        pub fn $method(&self) -> $crate::service::$mod::$ver::$vty<'_> {
            $crate::service::$mod::$ver::$vty::new(&self.config)
        }
    };
}

/// Define a card struct with one optional field per Lark locale.
///
/// Lark cards carry per-locale content across the same 16 locales. This macro
/// generates the struct with each locale as an `Option<$value>` field (skipped
/// when `None`), so the 16-field shape lives in one place instead of being
/// repeated per struct.
///
/// With a trailing `set`, it also emits a fluent `pub fn <locale>(self, value)`
/// setter for every locale.
///
/// ```ignore
/// // fields only
/// card_locale_struct!(MessageCardPlainTextI18n, String);
/// // fields plus per-locale setters
/// card_locale_struct!(I18nElements, Vec<Element>, set);
/// ```
macro_rules! card_locale_struct {
    ($(#[$meta:meta])* $name:ident, $value:ty) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Default, ::serde::Serialize, ::serde::Deserialize)]
        pub struct $name {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub zh_cn: Option<$value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub en_us: Option<$value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ja_jp: Option<$value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub zh_hk: Option<$value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub zh_tw: Option<$value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub id_id: Option<$value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub vi_vn: Option<$value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub th_th: Option<$value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub pt_br: Option<$value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub es_es: Option<$value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ko_kr: Option<$value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub de_de: Option<$value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub fr_fr: Option<$value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub it_it: Option<$value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ru_ru: Option<$value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub ms_my: Option<$value>,
        }
    };
    ($(#[$meta:meta])* $name:ident, $value:ty, set) => {
        card_locale_struct!($(#[$meta])* $name, $value);
        impl $name {
            pub fn new() -> Self {
                Self::default()
            }
            card_locale_struct!(@setter $value, zh_cn, en_us, ja_jp, zh_hk, zh_tw,
                id_id, vi_vn, th_th, pt_br, es_es, ko_kr, de_de, fr_fr, it_it,
                ru_ru, ms_my);
        }
    };
    (@setter $value:ty, $($locale:ident),+ $(,)?) => {
        $(
            pub fn $locale(mut self, value: $value) -> Self {
                self.$locale = Some(value);
                self
            }
        )+
    };
}

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
        impl $crate::service::common::FromRawResponse<$data> for $name {
            fn from_raw_response(
                api_resp: $crate::resp::ApiResp,
                raw: $crate::resp::RawResponse<$data>,
            ) -> Self {
                Self {
                    api_resp,
                    code_error: raw.code_error,
                    data: raw.data,
                }
            }
        }
    };
    ($name:ident) => {
        #[derive(Debug, Clone)]
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
        impl $crate::service::common::FromV2Response<serde_json::Value> for $name {
            fn from_v2_response(
                api_resp: $crate::resp::ApiResp,
                code_error: Option<$crate::resp::CodeError>,
                data: Option<serde_json::Value>,
            ) -> Self {
                Self {
                    api_resp,
                    code_error,
                    data,
                }
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
        impl $crate::service::common::FromV2Response<$data> for $name {
            fn from_v2_response(
                api_resp: $crate::resp::ApiResp,
                code_error: Option<$crate::resp::CodeError>,
                data: Option<$data>,
            ) -> Self {
                Self {
                    api_resp,
                    code_error,
                    data,
                }
            }
        }
    };
}

/// Generate [`EventDispatcher`](crate::event::EventDispatcher) registration
/// methods for typed event payloads.
///
/// Each entry expands to a builder method that deserializes the raw event JSON
/// into the given payload type and forwards it to the user handler. Two forms
/// are accepted:
///
/// - single event key:    `on_method => PayloadType : "event.key"`
/// - multiple event keys: `on_method => PayloadType : ["k1", "k2", ...]`
///   (one handler routed to several keys; requires the handler to be `Clone`)
///
/// ```ignore
/// event_handlers! {
///     /// Doc comments and other attributes are forwarded to the method.
///     on_p2_minutes_minute_generated_v1 => P2MinuteGeneratedV1
///         : "minutes.minute.generated_v1",
///     on_p1_user_changed_v3 => P1UserChangedV3
///         : ["user_add", "user_leave", "user_update"],
/// }
/// ```
macro_rules! event_handlers {
    (
        $(
            $(#[$meta:meta])*
            $method:ident => $payload:ty : $keys:tt
        ),* $(,)?
    ) => {
        impl $crate::event::EventDispatcher {
            $(
                event_handlers!(@method $(#[$meta])* $method => $payload : $keys);
            )*
        }
    };

    // Single event key: move the handler straight into one registration.
    (@method $(#[$meta:meta])* $method:ident => $payload:ty : $key:literal) => {
        $(#[$meta])*
        pub fn $method<F, Fut>(self, handler: F) -> Self
        where
            F: Fn($payload) -> Fut + Send + Sync + 'static,
            Fut: ::std::future::Future<Output = ::std::result::Result<(), $crate::error::LarkError>>
                + Send
                + 'static,
        {
            self.on_event($key, $crate::events::common::wrap_handler(handler))
        }
    };

    // Multiple event keys: route one `Clone` handler to every key.
    (@method $(#[$meta:meta])* $method:ident => $payload:ty : [ $($key:literal),+ $(,)? ]) => {
        $(#[$meta])*
        pub fn $method<F, Fut>(self, handler: F) -> Self
        where
            F: Fn($payload) -> Fut + Send + Sync + Clone + 'static,
            Fut: ::std::future::Future<Output = ::std::result::Result<(), $crate::error::LarkError>>
                + Send
                + 'static,
        {
            event_handlers!(@register_many self, handler, $($key),+)
        }
    };

    (@register_many $dispatcher:expr, $handler:ident, $key:literal) => {
        $dispatcher.on_event($key, $crate::events::common::wrap_handler($handler))
    };

    (@register_many $dispatcher:expr, $handler:ident, $key:literal, $($rest:literal),+) => {
        event_handlers!(
            @register_many
            $dispatcher.on_event($key, $crate::events::common::wrap_handler($handler.clone())),
            $handler,
            $($rest),+
        )
    };
}
