## Advent of Code, 2021

Return to rust, baby! After a stint in f#, I want to go through this in Rust again.

### Notable files

`src/bin/dayx.rs` is my template file.
`src/bin/day0.rs` is a previous year's problem using the template
to make sure that it works as expected.

`src/bin/reference_memoize.rs` is an example of using memoization in Rust.
(This is tricky and useful having a reference for since Rust doesn't allow unguarded mutable
access to static variables by default.)
`src/bin/day6.rs` also has an example of using memoization.
