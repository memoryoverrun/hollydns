#[macro_export]
macro_rules! derive_enum_conversions {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$vmeta:meta])*
                $variant:ident $(= $val:expr)?
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $(
                $(#[$vmeta])*
                $variant $(= $val)?,
            )*
        }

        impl $name {
            // 获取所有可能的变体名称
            pub fn variant_names() -> &'static [&'static str] {
                &[
                    $(stringify!($variant)),*
                ]
            }

            // 将枚举转换为字符串
            pub fn to_string(&self) -> String {
                match self {
                    $(Self::$variant => stringify!($variant).to_string()),*
                }
            }

            // 获取枚举值对应的数字
            pub fn to_number(&self) -> i32 {
                match self {
                    $(Self::$variant => {
                        let val: i32 = derive_enum_conversions!(@get_value $variant $(, $val)?);
                        val
                    }),*
                }
            }
        }

        // 从字符串转换为枚举
        impl std::str::FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.to_uppercase().as_str() {
                    $(s if s == stringify!($variant).to_uppercase() => Ok(Self::$variant),)*
                    _ => Err(format!("Unknown variant: {}", s))
                }
            }
        }

        // 从整数转换为枚举
        impl From<i32> for $name {
            fn from(value: i32) -> Self {
                match value {
                    $(v if v == derive_enum_conversions!(@get_value $variant $(, $val)?) => Self::$variant,)*
                    _ => panic!("Invalid value: {}", value)
                }
            }
        }
    };

    // 内部规则：获取枚举值
    (@get_value $variant:ident) => {
        {
            static mut COUNTER: i32 = 0;
            unsafe {
                let current = COUNTER;
                COUNTER += 1;
                current
            }
        }
    };

    (@get_value $variant:ident, $val:expr) => {
        $val
    };
}

// 使用示例
#[cfg(test)]
mod tests {

    derive_enum_conversions! {
        #[derive(Debug, PartialEq)]
        pub enum Status {
            Active = 1,
            Inactive,  // 将自动递增为2
            Pending = 5,
            Deleted   // 将自动递增为6
        }
    }

    #[test]
    fn test_from_string() {
        assert_eq!(Status::from_str("Active").unwrap(), Status::Active);
        assert_eq!(Status::from_str("ACTIVE").unwrap(), Status::Active);
        assert_eq!(Status::from_str("active").unwrap(), Status::Active);
        assert!(Status::from_str("Unknown").is_err());
    }

    #[test]
    fn test_from_number() {
        assert_eq!(Status::from(1), Status::Active);
        assert_eq!(Status::from(2), Status::Inactive);
        assert_eq!(Status::from(5), Status::Pending);
        assert_eq!(Status::from(6), Status::Deleted);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Status::Active.to_string(), "Active");
        assert_eq!(Status::Inactive.to_string(), "Inactive");
    }

    #[test]
    fn test_to_number() {
        assert_eq!(Status::Active.to_number(), 1);
        assert_eq!(Status::Inactive.to_number(), 2);
        assert_eq!(Status::Pending.to_number(), 5);
        assert_eq!(Status::Deleted.to_number(), 6);
    }
}