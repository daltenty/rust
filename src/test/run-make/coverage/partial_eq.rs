// This test confirms an earlier problem was resolved, supporting the MIR graph generated by the
// structure of this test.

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    major: usize,
    minor: usize,
    patch: usize,
}

impl Version {
    pub fn new(major: usize, minor: usize, patch: usize) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

fn main() {
    let version_3_2_1 = Version::new(3, 2, 1);
    let version_3_3_0 = Version::new(3, 3, 0);

    println!("{:?} < {:?} = {}", version_3_2_1, version_3_3_0, version_3_2_1 < version_3_3_0);
}

/*

This test verifies a bug was fixed that otherwise generated this error:

thread 'rustc' panicked at 'No counters provided the source_hash for function:
    Instance {
        def: Item(WithOptConstParam {
            did: DefId(0:101 ~ autocfg[c44a]::version::{impl#2}::partial_cmp),
            const_param_did: None
        }),
        substs: []
    }'
The `PartialOrd` derived by `Version` happened to generate a MIR that generated coverage
without a code region associated with any `Counter`. Code regions were associated with at least
one expression, which is allowed, but the `function_source_hash` was only passed to the codegen
(coverage mapgen) phase from a `Counter`s code region. A new method was added to pass the
`function_source_hash` without a code region, if necessary.

*/