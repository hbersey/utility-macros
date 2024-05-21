# utility-macros
A Rust library to emulate [Utility Types](https://www.typescriptlang.org/docs/handbook/utility-types.html) and [Unions](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#union-types) from TypeScript

## Attributes

### Container Attributes
```rust
#[utility_macros(name="Name")]
#[utility_macros(case_all="UPPER_SNAKE")]
#[utility_macros(derive="Debug, Clone")]
#[utility_macros(where="T: Debug + Clone")]
```

### Field / Variant Attributes
```rust
#[utility_macros(name="Name")]
#[utility_macros(case="UPPER_SNAKE")]
#[utility_macros(skip)]
```

Coming Soon:
```rust
#[utility_macros(default)] 
#[utility_macros(default="value")] or #[utility_macros(default=123)] 
```

### Deriving multiple traits
Coming Soon... in some similar format:
```rust
#[utility_macros(partial=(name="Name", case="UPPER_SNAKE", derive="Debug, Clone"), required=(name="Name2"))]
```