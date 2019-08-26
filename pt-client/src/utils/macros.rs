#[macro_export]
macro_rules! yellow {
    ( $( $x:expr ),* ) => {
        {
            Color::Light(BaseColor::Yellow)
        }
    };
}