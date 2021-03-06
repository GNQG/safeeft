# safeeft
Safe and branchless error-free transformation algorithms for floating point numbers.

Now supports types which impl `num_traits::Float` and size of bits of its significand is odd (Because `Float` does not offer its size).

[Documents](https://docs.rs/safeeft)

## Benchmark

With nightly compiler, execute

`$ cargo +nightly bench`

If your CPU has `fma` target-feature,

`$ RUSTFLAGS='-C target-cpu=native' cargo +nightly bench --features use-fma`

If your CPU does not have `fma` and compile and run with this command, `***_fma` will work very slowly due to software emulation of `fma`.

### Sample result

* compiler: rustc 1.23.0-nightly
* CPU: Intel Core i5-4570(Haswell)@3.20GHz

#### `twosum`

| algorithm             | time (ns) / operation |
|-----------------------|-----------------------|
| `twosum`(not safe)    |                1.7530 |
| `safetwosum_branch`   |                4.4827 |
| `safetwosum_straight` |                4.6438 |
| `safetwosum_fma`      |                2.4749 |

#### `split`

| algorithm                    |  time (ns) / operation |
|------------------------------|------------------------|
| `split`(not safe)            |                 0.8915 |
| `safesplit_branch`(not safe) |                 1.1918 |
| `safesplit_straight`         |                 3.3454 |

#### `twoproduct`

| algorithm                 | time (ns) / operation |
|---------------------------|-----------------------|
| `twoproduct`(not safe)    |                2.9322 |
| `safetwoproduct_branch`   |                3.7826 |
| `safetwoproduct_straight` |               12.9883 |
| `safetwoproduct_fma`      |                0.7125 |

