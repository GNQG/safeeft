# safeeft
Safe and branchless error-free transformation algorithms for floating point numbers.

Now supports only `f64`.

[Documents](https://docs.rs/safeeft)

## Benchmark

`$cargo +nightly bench`

If x86 with `fma`,

`$cargo +nightly bench --features use-fma`

### `twosum`

| algorithm             | time (ns) / operation |
|-----------------------|-----------------------|
| `twosum`(not safe)    |                1.8391 |
| `safetwosum_branch`   |                4.6489 |
| `safetwosum_straight` |                4.8499 |
| `safetwosum_fma`      |                3.9599 |

### `split`

| algorithm                    |  time (ns) / operation |
|------------------------------|------------------------|
| `split`(not safe)            |                 0.9141 |
| `safesplit_branch`(not safe) |                 1.1899 |
| `safesplit_straight`         |                 3.4384 |

### `twoproduct`

| algorithm                 | time (ns) / operation |
|---------------------------|-----------------------|
| `twoproduct`(not safe)    |                2.9827 |
| `safetwoproduct_branch`   |                3.9190 |
| `safetwoproduct_straight` |               13.5769 |
| `safetwoproduct_fma`      |                0.9225 |

