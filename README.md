Comrades, for too long have the capitalist pigs resorted to `O(n)` cloning of strings to maintain their "ownership" lifestyles.
And woe, for [`Rc<T>`](alloc::rc::Rc) and [`Arc<T>`](alloc::sync::Arc) are still beholden to the cruel rule of forced allocation.
But rejoice! For this crate presents a truly socialist shared string type, fully customizable by the People and for the People.

This crate introduces two new generic types, [`OurString`] and [`OurBytes`], which are customizable shared string/bytes types with (allocation-free) auto-inlining for small data.
As shared types, these values are immutable and cloning is `O(1)`.

The first generic parameter is the (shared) [`Comrade`] type, such as [`Rc<T>`](alloc::rc::Rc) or [`Arc<T>`](alloc::sync::Arc) for any `T` that is constructable from `&[u8]` and derefs to `[u8]`.
Notably, this includes `Rc<Vec<u8>>`, `Rc<[u8]>`, `Arc<Vec<u8>>`, and `Arc<[u8]>`.
You may also use other specialized types defined in this crate, such as [`RcBytes`](crate::comrades::RcBytes) and [`ArcBytes`](crate::comrades::ArcBytes), or even implement [`Comrade`] on your own container type.

The second generic parameter is the max inlining size.
Increasing this value allows larger strings to be stored inline (i.e., without allocations), but also increases the size of the struct overall.
Note that inlining is limited to 254 bytes, even if you make the stated max size larger.

## Examples

For example, we can use these types to make a shared string type with the same size as `String` but which inlines up to 23 bytes, much like some other crates:

```
# use std::rc::Rc;
# use our_string::OurString;
type MyString = OurString<Rc<[u8]>, 23>;
assert_eq!(size_of::<MyString>(), size_of::<String>());

let a = MyString::from("hello world!");
assert_eq!(a, "hello world!");
```

However, you may notice that `Option<MyString>` is larger than `MyString` (unlike `String`), but we can easily trade one byte of inlining for this size optimization:

```
# use std::rc::Rc;
# use our_string::OurString;
type MyString = OurString<Rc<[u8]>, 22>;
assert_eq!(size_of::<Option<MyString>>(), size_of::<String>());

let a = MyString::from("hello world!");
assert_eq!(a, "hello world!");
```

In the previous examples, we use `Rc<[u8]>` rather than `Rc<Vec<u8>>` to avoid double allocation/indirection in the non-inline case.
However, `Rc<[u8]>` takes up 16 bytes on the stack, rather than `Rc<Vec<u8>>` at 8 bytes.
Thus, if the size of your struct is more important than the possible double indirection, you may instead use something like:

```
# use std::rc::Rc;
# use our_string::OurString;
type MyString = OurString<Rc<Vec<u8>>, 15>; // double allocation/indirection
assert_eq!(size_of::<MyString>(), 16);
assert_eq!(size_of::<String>(), 24);

let a = MyString::from("hello world!");
assert_eq!(a, "hello world!");
```

But even this double allocation/indirection can be circumvented by switching our comrade type to a specialized one provided by this crate:

```
# use our_string::comrades::RcBytes;
# use our_string::OurString;
type MyString = OurString<RcBytes, 15>; // no double allocation/indirection
assert_eq!(size_of::<MyString>(), 16);
assert_eq!(size_of::<String>(), 24);

let a = MyString::from("hello world!");
assert_eq!(a, "hello world!");
```

In general, you should always prefer `RcBytes` over `Rc<[u8]>` unless you actually need to deal with `Rc<[u8]>` values directly.
Similarly, you should always prefer `ArcBytes` over `Arc<[u8]>`.

This is all to say that the important advantage of this crate is that you have all the control in how the bytes/string type is laid out, and the api remains completely unchanged.

- If you want more inlining, increase the max inlining size.
- If you want to save space, decrease the max inlining size.
- If you want to avoid double indirection, use `RcBytes` or `Rc<[u8]>`.
- If you want to minimize the struct size, use `RcBytes` or `Rc<Vec<u8>>`.
- If you want thread safety, use `ArcBytes` or `Arc<T>`.
- If you want to use a custom bytes type internally, go right ahead and use `YourOwnComrade` or `Rc<YourOwnString>`.

The choice is yours, comrade.

## `no_std`

This crate supports building in `no_std` environments out of the box.
Naturally, `alloc` is still required.
