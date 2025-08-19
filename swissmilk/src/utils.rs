// https://stackoverflow.com/a/57578431
#[macro_export]
macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<i32> for $name {
            type Error = String;
            
            #[inline(always)]
            fn try_from(value: i32) -> Result<Self, Self::Error> {
                match value {
                    $(x if x == $name::$vname as i32 => Ok($name::$vname),)*
                    _ => Err("Couldn't convert in to enum.".to_string()),
                }
            }
        }
    }
}
