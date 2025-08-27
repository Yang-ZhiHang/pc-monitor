#[macro_export]
macro_rules! singleton_with_logging {
    ($struct_name:ty, $instance_name:ident) => {
        static $instance_name: std::sync::OnceLock<$struct_name> = std::sync::OnceLock::new();

        impl $struct_name {
            pub fn global() -> &'static Self {
                $instance_name.get_or_init(|| {
                    let instance = Self::new();
                    $crate::logging!(
                        debug,
                        $crate::utils::logging::Type::Setup,
                        true,
                        "{} initialized.",
                        stringify!($struct_name)
                    );
                    instance
                })
            }
        }
    };
}
