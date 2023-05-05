
# Mathx

A mathematics library that is compatible with `no_std`, to be used with embedded systems. Giving back some functionality lost when removing the standard library.

Structures that are fully usable and stable:
- Math (Used for `no_std` to compensate for missing features)
- Vectors (`Vector2`, `Vector3`)

Full Documentation: https://docs.rs/mathx

**Note**: This library is still a work in progress.

## Math (struct)

This struct is more of a "static" struct where it contains all the math functions that are included with the standard library plus a little more. It basically uses the default implementation and switches to an approximation when using the `no_std` feature so the functionality will still be there regardless. Below is an example of how it can be substituted:

```rs
let x = 1.0_f32;

// This will cause a compilation error if using #![no_std]
println!("X: {}", x.cos());

// Use the version inside Math as it will work if you're using #![no_std] or not
println!("X: {}", Math::cos(x));
```

Full Documentation for this struct: https://docs.rs/mathx/latest/mathx/struct.Math.html

## Use with `no_std` Environment

This library is friendly with a `no_std` environment, although it is an opt-in feature you need to set to make it compatible with a `no_std` environment. With the standard library, the `f32` type holds math functionality such as `cos`, `log`, `powf`, etc. But without the standard library those math functions no longer exist. By default, this library tries to use those math functions as they are expected to be much faster and more accurate than this implementation. Add the `no_std` feature to have the library switch to an approximation of those math functions that tries to balance accuracy with performance intended for embedded systems, giving back that functionality. It avoids using any lookup tables, as spacial complexity could be a potential concern.
