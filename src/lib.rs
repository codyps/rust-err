#[macro_export]
macro_rules! some_error {
    ($call:expr) => (
        match $call {
            Some(e) => panic!("{} => {:?}", stringify!($call), e),
            None => ()
        }
    )
}

#[macro_export]
macro_rules! or_panic {
    ($e:expr) => ({
        let v : Result<_,_> = $e;
        let s = stringify!($e);
        or_panic!(v, "failure in {:?} = {:?}", s, v.err())
    });
    ($e:expr, $fmt:expr, $($arg:tt)+) => (
        match $e {
            Ok(e) => e,
            Err(_) => {
                panic!($fmt, $($arg)+)
            }
        }
    );
}

#[macro_export]
macro_rules! from_error  {
    ($enum_base:ident => $enum_elem:ident($elem_type:ty)) => (
        impl ::std::error::FromError<$elem_type> for $enum_base {
            fn from_error(err: $elem_type) -> $enum_base {
                $enum_base::$enum_elem(err)
            }
        }
    )
}


#[cfg(test)]
mod test {

    #[derive(Debug)]
    pub enum AnError {
        Foo(&'static str)
    }

    from_error! { AnError => Foo(&'static str) }

    fn f() -> Result<(),AnError> {
        try!(Err("hi"))
    }

    #[test]
    fn it_works() {
        or_panic!(Result::Ok::<(),&'static str>(()));

        println!("Got: {:?}", f())
    }



}
