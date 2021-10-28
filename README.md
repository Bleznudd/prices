
# Prices
[![Rust](https://github.com/Bleznudd/prices/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/Bleznudd/prices/actions/workflows/rust.yml)

Manage prices of your products

## Target users

Do you live by buying and selling things? Do you need to keep track of the prices of your products, how much taxes you need to pay on them and the final selling prices? Do you like cli and unix like operating systems? Then, this could be the right tool for you.

## Installation

Install `cargo` and run:
```
cargo install --path .
```
from within the main folder.

## Usage

Prices helps you keep track of your product, and by default, if invoked without any argument, will just print them in a nice way
```
$ prices                                  
* Default
+ CAT&Co
- catnip            10.00€  10.0%  20.0%   13.20€
- scratching post   20.00€  10.0%  10.0%   24.20€
- food               5.00€  10.0%  30.0%    7.15€
+ BIRDstuff
- seeds              2.00€   4.0%  30.0%    2.70€
+ DOGcentre
- leash              5.00€   6.0%  30.0%    6.89€
- toy                3.00€  20.0%  20.0%    4.32€
- metal bowl         8.00€  10.0%  30.0%   11.44€
- food               6.00€  10.0%  20.0%    7.92€
```

To see how to add, search and remove products, which calculations are done automatically, and what are the features, please refer to the [usage](./USAGE.md) page.





## :warning: Disclaimer

This is my first Rust program, I'm still learning and I'm sure **a lot** of the code doesn't respect the standards.
If you're an experienced Rust programmer, willing to teach me where I'm wrong and why through an issue or a pull request, I'll happily take your help.