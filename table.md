|         | recursion | Advanced recursion* | Iteration  | Full-Size Matrices | Compact matrices |
| ------  | --------- | ------------------- | ---------  | ------------------ | ---------------- |
| C (GMP) | 40,42     | 72,74               | 600k, 800k | 8M,13M             | 70M,128M         |
| Rust 1  | 41,43     | 71,74               | 290k, 400k |                    | 45M, 70M         |
| Rust 2  | 41,43     | 71,74               | 550k, 800k |                    | 48M, 90M         |
| Rust 3  | 41,43     | 71,74               | 300k, 420k |                    | 10M, 16M         |
| Python  | 34,35     | 59,61               | 230k, 330k | 2M,2.4M            | 3M, 5M           |

bounds are half a second, one second

\* 2-step, 2 bonus guards
Rust 1 is with malachite, 2 is with rug, 3 is with num-bigint. Recursion and advanced recursion are always done with simple `u128`, because bigints aren't necessary.
