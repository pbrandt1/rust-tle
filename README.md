# rust-tle

_Finally,_ a TLE parser and generator for Rust.

A two-line element set (TLE) is a compact representation of a satellite's orbit. A TLE looks like this:

```
ISS (ZARYA)
1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537
```

**You might notice:**
- It's three lines long (not two)
- It's basically unreadable gibberish
- ZERO emojis. Literally none in that example 😑🙄

SO WITH THAT IN MIND

DON'T USE TLE'S UNLESS YOU'RE SUPPORTING LEGACY SOFTWARE

LIKE ONLY IF YOU ARE LITERALLY USING PUNCH CARDS

New software should use something like JSON or protobuf for serializing orbit data.

```rust
let s = String::from("ISS (ZARYA)\n1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927\n2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537");
let iss_tle = tle::parse_tle(&s);

// Now you can access the fields
assert_eq!(iss_tle.name, "ISS (ZARYA)");

// Debug print is friendly
println!("{:?}", iss_tle);

// Display print is the original format (but might not be exact b/c floating point math)
println!("{}", iss_tle);

```

----------------

_Reference Material_

* [Wikipedia TLE article](https://en.wikipedia.org/wiki/Two-line_element_set#Format)
* [Celestrak's TLE guide](https://celestrak.com/columns/v04n03/)

-----------------

[MIT LICENSE](LICENSE)
