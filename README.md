# Dyll - Dynamically Linked Shared Library Stub Generator

**Dyll** 🥒 generates drop-in replacement shared libraries for Linux with
function interception/mocking capabilities.

## Why, What, How 

mocking out shared libraries typically comes in two forms:
1. dynamic binary instrumentation
1. compiled mock libraries

dynamic instrumentation is great, but is a deep rabbit-hole which requires a lot
of architecture specific awareness. tools like [frida](https://frida.re/) and
[capstone](https://github.com/capstone-engine/capstone) walk this line very
skillfully, but bring lots of complexity and additional toolchain overhead to
the equation.

if we want to go down the compiled mock route, we need to adhere to the
all-knowning C-ABI, since that is the de-facto FFI boundary that our library has
to expose in order to actually pass/recieve data through proper calling
conventions. one problem here is that we dont always know the full API spec
(function signatures, type definitons) of the library we're trying to mock, and
we need all of this information at compile-time, otherwise we could get missing
symbols, segfaults, undefined-behavior, and more. having said this, creating
compiled, drop-in replacement mocks gets hairy when the target has any of the
following requirements:
1. the library gets resolved via `dlopen` and/or `dlopen + dlsym` rather than
   standard linker resolution
1. callers expect identical environment between the mock and the original (e.g.
   containerization and/or separate mount namespaces)

to address some of this complexity, `dyll` takes a different approach from the
typical `LD_PRELOAD` or `dlopen` wrappers for shared library indirection. it
consumes a target `.so` library and embeds it completely into its own binary,
and at runtime calls `dlopen` on this copy via a `memfd` handle inside its own
process. this avoids the tracking of additional files and still allows `dyll` to
forward calls to the original library.

> [!NOTE]
> `dyll` currently doesn't make you provide the real `.so`, if you just embed
> `/dev/null` it will recognize the library cannot be loaded and returns a noop
> stub instead.

through additional codegen, `dyll` provides hooks to register custom logic for
any of the generated pass-through stubs, as well as a basic level of
observability to trace library calls.

<details>
<summary>Example Scenario</summary>

this process roughly translates `C` code to `Rust` like so:

```c
int add(int a, int b);
```

```rust
pub mod add {
    pub static mut HANDLE: Option<&'static dyn Fn(unsafe extern "C" fn(a: c_int, b: c_int) -> c_int, c_int, c_int) -> c_int = None;
    pub fn register_handler(handle: &'static impl Fn(unsafe extern "C" fn(a: c_int, b: c_int) -> c_int, c_int, c_int) -> c_int) {
        unsafe { HANDLE = Some(handle) }
    }
}
pub unsafe extern "C" fn add(a: c_int, b: c_int) -> c_int {
    let span = tracing::span!(tracing::Level::TRACE, stringify!(add));
    let _enter = span.enter();
    tracing::trace!("@entry");
    let add: extern "C" fn(a: c_int, b: c_int) -> c_int = unsafe { std::mem::transmute(get_sym("add")) };
    let ret = match unsafe { add::HANDLE } {
        Some(handler) => handler(add, a, b),
        None => add(a, b),
    };
    tracing::trace!("@exit");
    ret
}
```

usable inside the mock scaffold via the following:

```rust
add::register_handler(&|add, a, b| {
    tracing::info!("hello from mock!");
    tracing::info!("adding {} and {}", a, b);
    let result = unsafe { add(a, b) };
    tracing::info!("original returned: {}", result);
    return result;
});
```

running this code through another programs would produce logging like:

```log
2026-07-20 TRACE add: mock: @entry
2026-07-20 INFO add: mock: hello from mock!
2026-07-20 INFO add: mock: adding 1 and 2
2026-07-20 INFO add: mock: original returned: 3
2026-07-20 TRACE add: mock: @exit
```

</details>

## Reverse-Engineering

`dyll` currently has no contribution to library reverse-engineering efforts. if
you have a `.so` without the associated headers, but valid `DWARF` metadata, you
can use tools like [dwarf2cpp ](https://github.com/EndstoneMC/dwarf2cpp) to
parse out valid headers. if the debug data is not present, you could try
generating headers with a decompiler.
