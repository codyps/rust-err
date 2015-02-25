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

#[macro_export]
macro_rules! auto_error_enum {
    (enum $enum_name:ident { $( $elem:ident ( $elem_type:ty ) ),* }) => (
        #[derive(Debug)]
        enum $enum_name { $($elem($elem_type)),* }

        $(
        impl ::std::error::FromError<$elem_type> for $enum_name {
            fn from_error(err: $elem_type) -> $enum_name {
                $enum_name::$elem(err)
            }
        }
        )*
    )
}

#[macro_export]
macro_rules! error_enum {
    (auto $enum_name:ident $elem:ident ( $elem_type:ty ) ) => (
        impl ::std::error::FromError<$elem_type> for $enum_name {
            fn from_error(err: $elem_type) -> $enum_name {
                $enum_name::$elem(err)
            }
        }
    );

    (bare $enum_name:ident $elem:ident ( $($elem_type:ty),*) ) => (
    );

    (enum $enum_name:ident { $($elem_kind:ident $elem:ident ( $($elem_type:ty),* ) ),* }) => (
        #[derive(Debug)]
        enum $enum_name { $($elem($($elem_type),*)),* }

        $(error_enum!{$elem_kind $enum_name $elem ( $($elem_type),* ) })*
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

    mod auto {
        auto_error_enum! {enum FooEnum {
            Foo(&'static str)
        }}

        fn y() -> Result<(),FooEnum> {
            try!(Err("hi"))
        }

        #[test]
        fn test() {
            println!("Got: {:?}", y());
        }
    }

    mod ctrl {
        error_enum! {enum GenEnum {
            auto Foo(&'static str)
        }}

        fn x() -> Result<(),GenEnum> {
            try!(Err("hi"))
        }

        #[test]
        fn test() {
            println!("Got: {:?}", x());
        }
    }
}
