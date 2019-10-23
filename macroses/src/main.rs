#![feature(type_ascription)]

mod macroses;
mod responses;

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_three_test() {
        assert_eq!(just_three!(), 3);
    }

    #[test]
    fn map_test() {
        use std::collections::HashMap;
        let m = map![(1 => 3),(2 => 4)];
        assert_eq!(*m.get(&1).unwrap(), 3);
        assert_eq!(*m.get(&2).unwrap(), 4);
        assert_eq!(*m.get(&3).unwrap_or(&0), 0);
    }

    #[test]
    fn multiply_test() {
        use std::collections::HashMap;
        let m: i32 = multiply!(1,2,3,12);
        assert_eq!(m, 1 * 2 * 3 * 12);
        assert_eq!(multiply!(1.0,2.0,3.0): f32, 6.0);
    }

    #[test]
    fn cfg_test() {
        assert_eq!(cfg!(windows), true);
        assert_eq!(cfg!(unix), false);
    }

    #[test]
    fn functor_test() {
        assert_eq!(functor![23 => |x|x+1], 24);
    }

    #[test]
    fn repeat_test() {
        let mut x = 0;
        repeat![10, {x+=1}];
        assert_eq!(x, 10);
    }

    #[test]
    fn make_fn_test() {
        make_fn!(fr,{assert!(true)});
    }

    #[test]
    fn make_fn1_test() {
        make_fn1!(fr,x, i32,{x+1});
        assert_eq!(fr(10),11);
    }
}
