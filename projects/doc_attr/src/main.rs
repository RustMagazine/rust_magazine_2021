macro_rules! make_function {
    ($name:ident, $value:expr) => {
        // 这里使用 concat! 和 stringify! 构建文档注释
        #[doc = concat!("The `", stringify!($name), "` example.")]
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!(
            "assert_eq!(", module_path!(), "::", stringify!($name), "(), ",
            stringify!($value), ");")
        ]
        /// ```
        pub fn $name() -> i32 {
            $value
        }
    };
}


make_function! {func_name, 123}

fn main() {
    func_name();
}