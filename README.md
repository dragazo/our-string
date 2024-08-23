Comrades, for too long have the capitalist pigs resorted to `O(n)` cloning of strings to maintain their "ownership" lifestyles.
And woe, for [`Rc<T>`](alloc::rc::Rc) and [`Arc<T>`](alloc::sync::Arc) are still beholden to the dictatorship of a global allocator.
But rejoice! For this crate presents a truly socialist shared string type, fully customizable by the People and for the People.

This safe-only crate introduces two new generic types, [`OurString`] and [`OurBytes`], which are customizable shared string/bytes types with auto-inlining for small data.
As shared types, these values are immutable and cloning is `O(1)`.

The first generic parameter is the (shared) "comrade" type, which can be [`Rc<T>`](alloc::rc::Rc) or [`Arc<T>`](alloc::sync::Arc) for any `T` that is convertible to/from `str` and `[u8]`, respectively.
Notably, this includes `String` and `str` for [`OurString`], as well as `Vec<u8>` and `[u8]` for [`OurBytes`].

The second generic parameter is the max inlining size.
Increasing this value allows larger strings to be stored inline (i.e., without allocations), but also increases the size of the struct overall.
Note that inlining is limited to 255 bytes, even if you make the stated max size larger.

## Examples

For example, we can use these types to make a shared string type with the same size as [`String`](alloc::string::String) but which inlines up to 23 bytes, much like some other crates:

```
# use std::rc::Rc;
# use our_string::OurString;
type MyString = OurString<Rc<str>, 23>;
assert_eq!(size_of::<MyString>(), size_of::<String>());

let a = MyString::from("hello world!");
assert_eq!(a, "hello world!");
```

However, you may notice that `Option<MyString>` is larger than `MyString` (unlike `String`), but we can easily trade one byte of inlining for this size optimization:

```
# use std::rc::Rc;
# use our_string::OurString;
type MyString = OurString<Rc<str>, 22>;
assert_eq!(size_of::<Option<MyString>>(), size_of::<String>());

let a = MyString::from("hello world!");
assert_eq!(a, "hello world!");
```

In the previous examples, we use `Rc<str>` rather than `Rc<String>` to avoid double indirection in the non-inline case.
However, `Rc<str>` takes up 16 bytes, rather than `Rc<String>` at 8 bytes.
Thus, if the size of your struct is more important than the possible double indirection, you may instead use something like:

```
# use std::rc::Rc;
# use our_string::OurString;
type MyString = OurString<Rc<String>, 15>;
assert_eq!(size_of::<MyString>(), 16);
assert_eq!(size_of::<String>(), 24);

let a = MyString::from("hello world!");
assert_eq!(a, "hello world!");
```

The important advantage of this crate is that you have all the control in how the string type is laid out, and the api remains completely unchanged.

- If you want more inlining, increase the max inlining size.
- If you want to save space, decrease the max inlining size.
- If you want to avoid double indirection at the expense of some struct size, use `Rc<str>`.
- If you want to minimize the struct size as much as possible, use `Rc<String>`.
- If you want thread safety, use `Arc<T>` instead of `Rc<T>`.
- If you want to use a custom string type internally, go right ahead and use `Rc<WhateverYouWant>`.

The choice is yours.

## `no_std`

This crate supports building in `no_std` environments out of the box.
