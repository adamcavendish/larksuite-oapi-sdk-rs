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
