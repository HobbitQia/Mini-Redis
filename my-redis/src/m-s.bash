cargo run --bin server redis-master-1  > /dev/null 2>&1 &
cargo run --bin server redis-slave-1  > /dev/null 2>&1 &
cargo run --bin server redis-master-2  > /dev/null 2>&1 &
cargo run --bin server redis-slave-2  > /dev/null 2>&1 &
cargo run --bin server proxy  > /dev/null 2>&1 &
