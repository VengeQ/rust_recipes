extern crate num_traits;
use std::collections::HashMap;

#[macro_export]
macro_rules! just_three{
    () => {3}
}

#[macro_export]
macro_rules! multiply{
    ($($v:expr),*) =>{
        {
            let mut x = num_traits::identities::one();
            $(x=x*$v;)*
            x
        }
    };
}

#[macro_export]
macro_rules! map{
    ($(
        ($k:expr => $v:expr)
    ),*
    ) => {
            {
            let mut m = HashMap::new();
            $(m.insert($k,$v);)*
            m
        }

    };
}

#[macro_export]
macro_rules! functor{
    ($s:expr => $f:expr) => {$f($s)}
}

#[macro_export]
macro_rules! repeat{
    ($n:expr, $f:expr) => {
        for _ in 0.. $n{
            $f();
        }
    }
}

#[macro_export]
macro_rules! make_fn1{
    ($name: ident, $arg: ident, $t: tt, $body: block) => {
        pub fn $name ($arg: $t) -> $t $body
    }
}

#[macro_export]
macro_rules! make_fn {
    ($i: ident, $body: block) => {
        fn $i () $body
    }
}