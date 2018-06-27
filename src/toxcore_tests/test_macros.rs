//! File with testing macros. **Use only in tests!**


/** Implement `Arbitrary` trait for given struct with bytes.

E.g.

```
#[cfg(test)]
use ::quickcheck::Arbitrary;

struct Name(Vec<u8>);

#[cfg(test)]
impl_arb_for_bytes!(Name, 100);
```
*/
// FIXME: ↑ make it a real test, since doctest doesn't work
macro_rules! impl_arb_for_bytes {
    ($name: ident, $len: expr) => (
        impl Arbitrary for $name {
            fn arbitrary<G: Gen>(g: &mut G) -> Self {
                let n = g.gen_range(0, $len + 1);
                let mut bytes = vec![0; n];
                g.fill_bytes(&mut bytes[..n]);
                $name(bytes)
            }
        }
    )
}


/** Implement `Arbitrary` for given struct containing only `PackedNodes`.

E.g.

```
use ::quickcheck::Arbitrary;
use ::toxcore::dht::*;

struct Nodes(Vec<PackedNode>);

impl_arb_for_pn!(Nodes);
```
*/
// FIXME: ↑ make it a real test, since doctest doesn't work
macro_rules! impl_arb_for_pn {
    ($name:ident) => (
        impl Arbitrary for $name {
            fn arbitrary<G: Gen>(g: &mut G) -> Self {
                $name(Arbitrary::arbitrary(g))
            }
        }
    )
}

/** PublicKey from bytes. Returns `TestResult::discard()` if there are not
enough bytes.
*/
macro_rules! quick_pk_from_bytes {
    ($input:ident, $out:ident) => (
        if $input.len() < PUBLICKEYBYTES {
            return TestResult::discard()
        }

        let $out = PublicKey::from_slice(&$input[..PUBLICKEYBYTES])
            .expect("Failed to make PK from slice");
    )
}
