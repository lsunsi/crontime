# It's cron time! ⏲️
![badge](https://github.com/lsunsi/crontime/actions/workflows/test.yml/badge.svg)

### Description

This library is intended to work as a cron expression parser.
For now it yields an iterator of upcoming datetimes that follow the parsed expression, simple as that.

###### This library is in development. It does not implement the whole syntax yet.

### Usage

```rust
let now = datetime!(1917-11-07 00:00:00 UTC);
let ct = crontime::build(now, "1 * * * *").unwrap();

for dt in ct.take(3) {
    println!("{dt}");
}

// 1917-11-07 0:01:00.0 +00:00:00
// 1917-11-07 1:01:00.0 +00:00:00
// 1917-11-07 2:01:00.0 +00:00:00
```

### Thanks
