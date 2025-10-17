# compile-hashsig-to-risc0

## Instructions

```rust
curl -L https://risczero.com/install | bash
rzup install
rzup install rust 1.88.0
rustup default risc0
cargo build --target=riscv32im-risc0-zkvm-elf
```

Build log
```rust
   Compiling num-traits v0.2.19
   Compiling zerocopy v0.8.27
   Compiling generic-array v0.14.9
   Compiling quote v1.0.41
   Compiling getrandom v0.3.4
   Compiling typenum v1.19.0
   Compiling thiserror v2.0.17
   Compiling once_cell v1.21.3
   Compiling parking_lot_core v0.9.12
   Compiling crossbeam-epoch v0.9.18
error: target is not supported. You may need to define a custom backend see: https://docs.rs/getrandom/0.3.4/#custom-backend
   --> /home/kolby/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends.rs:204:9
    |
204 | /         compile_error!(concat!(
205 | |             "target is not supported. You may need to define a custom backend see: \
206 | |             https://docs.rs/getrandom/", env!("CARGO_PKG_VERSION"), "/#custom-backend"
207 | |         ));
    | |__________^

error[E0425]: cannot find function `fill_inner` in module `backends`
  --> /home/kolby/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/lib.rs:99:19
   |
99 |         backends::fill_inner(dest)?;
   |                   ^^^^^^^^^^ not found in `backends`

error[E0425]: cannot find function `inner_u32` in module `backends`
   --> /home/kolby/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/lib.rs:123:15
    |
123 |     backends::inner_u32()
    |               ^^^^^^^^^ not found in `backends`
    |
help: consider importing this function
    |
33  + use crate::util::inner_u32;
    |
help: if you import `inner_u32`, refer to it directly
    |
123 -     backends::inner_u32()
123 +     inner_u32()
    |

error[E0425]: cannot find function `inner_u64` in module `backends`
   --> /home/kolby/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/lib.rs:137:15
    |
137 |     backends::inner_u64()
    |               ^^^^^^^^^ not found in `backends`
    |
help: consider importing this function
    |
33  + use crate::util::inner_u64;
    |
help: if you import `inner_u64`, refer to it directly
    |
137 -     backends::inner_u64()
137 +     inner_u64()
    |

For more information about this error, try `rustc --explain E0425`.
error: could not compile `getrandom` (lib) due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
```

Cargo Tree
```rust
compile-hashsig-to-risc0 v0.1.0 (/home/kolby/ドキュメント/GitHub/compile-hashsig-to-risc0)
└── hashsig v0.1.0 (https://github.com/b-wagn/hash-sig?rev=1ed8d820636a2f34c1340e5748b90d3e03df556a#1ed8d820)
    ├── dashmap v6.1.0
    │   ├── cfg-if v1.0.4
    │   ├── crossbeam-utils v0.8.21
    │   ├── hashbrown v0.14.5
    │   ├── lock_api v0.4.14
    │   │   └── scopeguard v1.2.0
    │   ├── once_cell v1.21.3
    │   └── parking_lot_core v0.9.12
    │       ├── cfg-if v1.0.4
    │       ├── libc v0.2.177
    │       └── smallvec v1.15.1
    ├── num-bigint v0.4.6
    │   ├── num-integer v0.1.46
    │   │   └── num-traits v0.2.19
    │   │       [build-dependencies]
    │   │       └── autocfg v1.5.0
    │   └── num-traits v0.2.19 (*)
    ├── num-traits v0.2.19 (*)
    ├── p3-baby-bear v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba)
    │   ├── p3-field v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba)
    │   │   ├── itertools v0.14.0
    │   │   │   └── either v1.15.0
    │   │   ├── num-bigint v0.4.6 (*)
    │   │   ├── p3-maybe-rayon v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba)
    │   │   ├── p3-util v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba)
    │   │   │   └── serde v1.0.228
    │   │   │       ├── serde_core v1.0.228
    │   │   │       └── serde_derive v1.0.228 (proc-macro)
    │   │   │           ├── proc-macro2 v1.0.101
    │   │   │           │   └── unicode-ident v1.0.19
    │   │   │           ├── quote v1.0.41
    │   │   │           │   └── proc-macro2 v1.0.101 (*)
    │   │   │           └── syn v2.0.106
    │   │   │               ├── proc-macro2 v1.0.101 (*)
    │   │   │               ├── quote v1.0.41 (*)
    │   │   │               └── unicode-ident v1.0.19
    │   │   ├── paste v1.0.15 (proc-macro)
    │   │   ├── rand v0.9.2
    │   │   │   ├── rand_chacha v0.9.0
    │   │   │   │   ├── ppv-lite86 v0.2.21
    │   │   │   │   │   └── zerocopy v0.8.27
    │   │   │   │   └── rand_core v0.9.3
    │   │   │   │       └── getrandom v0.3.4
    │   │   │   │           ├── cfg-if v1.0.4
    │   │   │   │           └── libc v0.2.177
    │   │   │   └── rand_core v0.9.3 (*)
    │   │   ├── serde v1.0.228 (*)
    │   │   └── tracing v0.1.41
    │   │       ├── pin-project-lite v0.2.16
    │   │       ├── tracing-attributes v0.1.30 (proc-macro)
    │   │       │   ├── proc-macro2 v1.0.101 (*)
    │   │       │   ├── quote v1.0.41 (*)
    │   │       │   └── syn v2.0.106 (*)
    │   │       └── tracing-core v0.1.34
    │   ├── p3-mds v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba)
    │   │   ├── p3-dft v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba)
    │   │   │   ├── itertools v0.14.0 (*)
    │   │   │   ├── p3-field v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   │   │   ├── p3-matrix v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba)
    │   │   │   │   ├── itertools v0.14.0 (*)
    │   │   │   │   ├── p3-field v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   │   │   │   ├── p3-maybe-rayon v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba)
    │   │   │   │   ├── p3-util v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   │   │   │   ├── rand v0.9.2 (*)
    │   │   │   │   ├── serde v1.0.228 (*)
    │   │   │   │   ├── tracing v0.1.41 (*)
    │   │   │   │   └── transpose v0.2.3
    │   │   │   │       ├── num-integer v0.1.46 (*)
    │   │   │   │       └── strength_reduce v0.2.4
    │   │   │   ├── p3-maybe-rayon v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba)
    │   │   │   ├── p3-util v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   │   │   └── tracing v0.1.41 (*)
    │   │   ├── p3-field v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   │   ├── p3-symmetric v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba)
    │   │   │   ├── itertools v0.14.0 (*)
    │   │   │   ├── p3-field v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   │   │   └── serde v1.0.228 (*)
    │   │   ├── p3-util v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   │   └── rand v0.9.2 (*)
    │   ├── p3-monty-31 v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba)
    │   │   ├── itertools v0.14.0 (*)
    │   │   ├── num-bigint v0.4.6 (*)
    │   │   ├── p3-dft v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   │   ├── p3-field v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   │   ├── p3-matrix v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   │   ├── p3-maybe-rayon v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba)
    │   │   ├── p3-mds v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   │   ├── p3-poseidon2 v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba)
    │   │   │   ├── p3-field v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   │   │   ├── p3-mds v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   │   │   ├── p3-symmetric v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   │   │   ├── p3-util v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   │   │   └── rand v0.9.2 (*)
    │   │   ├── p3-symmetric v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   │   ├── p3-util v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   │   ├── paste v1.0.15 (proc-macro)
    │   │   ├── rand v0.9.2 (*)
    │   │   ├── serde v1.0.228 (*)
    │   │   ├── tracing v0.1.41 (*)
    │   │   └── transpose v0.2.3 (*)
    │   ├── p3-poseidon2 v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   ├── p3-symmetric v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   └── rand v0.9.2 (*)
    ├── p3-field v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    ├── p3-koala-bear v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba)
    │   ├── p3-field v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   ├── p3-monty-31 v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   ├── p3-poseidon2 v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   ├── p3-symmetric v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    │   └── rand v0.9.2 (*)
    ├── p3-symmetric v0.3.0 (https://github.com/Plonky3/Plonky3.git?rev=2117e4b#2117e4ba) (*)
    ├── rand v0.9.2 (*)
    ├── rayon v1.11.0
    │   ├── either v1.15.0
    │   └── rayon-core v1.13.0
    │       ├── crossbeam-deque v0.8.6
    │       │   ├── crossbeam-epoch v0.9.18
    │       │   │   └── crossbeam-utils v0.8.21
    │       │   └── crossbeam-utils v0.8.21
    │       └── crossbeam-utils v0.8.21
    ├── serde v1.0.228 (*)
    ├── sha3 v0.10.8
    │   ├── digest v0.10.7
    │   │   ├── block-buffer v0.10.4
    │   │   │   └── generic-array v0.14.9
    │   │   │       └── typenum v1.19.0
    │   │   │       [build-dependencies]
    │   │   │       └── version_check v0.9.5
    │   │   └── crypto-common v0.1.6
    │   │       ├── generic-array v0.14.9 (*)
    │   │       └── typenum v1.19.0
    │   └── keccak v0.1.5
    └── thiserror v2.0.17
        └── thiserror-impl v2.0.17 (proc-macro)
            ├── proc-macro2 v1.0.101 (*)
            ├── quote v1.0.41 (*)
            └── syn v2.0.106 (*)
```