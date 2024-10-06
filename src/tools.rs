pub trait ToUsize {
    fn to_usize(self) -> usize;
}

macro_rules! implToUsizeSigned {
    ($($t:ty),*) => {$(
            impl ToUsize for $t {
                fn to_usize(self) -> usize {
                    self.abs() as usize
                }
    })*};
}
macro_rules! implToUsizeUnsigned {
    ($($t:ty),*) => {$(
            impl ToUsize for $t {
                fn to_usize(self) -> usize {
                    self as usize
                }
            }
    )*};
}
implToUsizeUnsigned!(u8, u16, u32, u64, usize);
implToUsizeSigned!(i8, i16, i32, i64, isize);
