use std::str::FromStr;

pub trait UnwrapNextInt<T: FromStr>
{
    fn unwrap_next_int(&mut self) -> T where <T as FromStr>::Err: std::fmt::Debug;
}

impl<'a, T, U> UnwrapNextInt<T> for U
where
    T: FromStr,
    U: Iterator<Item=&'a str>
{
    fn unwrap_next_int(&mut self) -> T where <T as FromStr>::Err: std::fmt::Debug {
        self.next().unwrap().parse::<T>().unwrap()
    }
}
