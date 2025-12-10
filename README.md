# multiparse

Automatically parse multiple radices

## Usage

Have you ever wanted to take user input of an arbitrary radix? Well now you can!

```rust
"73".multiparse::<u8>()         // -> Ok(73)
"0b1001001".multiparse::<u8>()  // -> Ok(73)
"0o111".multiparse::<u8>()      // -> Ok(73)
"0x49".multiparse::<u8>()       // -> Ok(73)
```

Simply bring the `Multiparse` trait into scope:

```rust
use multiparse::Multiparse;
```

And then call the `multiparse` method anywhere that you could call `parse`.

The `Multiparse` trait is implemented for `Deref<Target = str>`, so it should work on everything that you would expect.