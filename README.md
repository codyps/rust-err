# err

A collection of macros for working with FromError, the most useful of which is `error_enum!`

```rust!
#[macro_use]
extern crate err;

error_enum! {
	enum MyError {
		bare SomethingCustom(&'static str),
		auto Io(std::io::Error)
	}

	enum MyOtherError {
		auto MyError(MyError)
	}
}

// An all the elements marked `auto` have auto-generated FromError
// implimentations created with the assumption that they are simple wrappers
// around a type.

```

## Limitations

 - No-arg enum elements are not supported
 - Trailing commas on the last enum element are not supported

