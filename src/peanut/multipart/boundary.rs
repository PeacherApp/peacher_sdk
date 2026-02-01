// Copyright 2021 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use rand::{Rng, SeedableRng, rngs::SmallRng};

/// A `BoundaryGenerator` is a policy to generate a random string to use
/// as a part boundary.
///
/// The default generator will build a random string of 6 ascii characters.
/// If you need more complexity, you can implement this, and use it with
/// [`Form::new`].
pub trait BoundaryGenerator {
    /// Generates a String to use as a boundary.
    fn generate_boundary() -> String;
}

pub(crate) struct RandomAsciiGenerator;

impl BoundaryGenerator for RandomAsciiGenerator {
    fn generate_boundary() -> String {
        let mut rng = SmallRng::from_os_rng();

        let a = rng.random::<u64>();
        let b = rng.random::<u64>();
        let c = rng.random::<u64>();
        let d = rng.random::<u64>();
        let e = rng.random::<u64>();
        let f = rng.random::<u64>();
        let g = rng.random::<u64>();
        let h = rng.random::<u64>();

        format!(
            "{:016x}-{:016x}-{:016x}-{:016x}-{:016x}-{:016x}-{:016x}-{:016x}",
            a, b, c, d, e, f, g, h
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{BoundaryGenerator, RandomAsciiGenerator};

    #[test]
    fn generate_random_boundary_not_empty() {
        assert!(!RandomAsciiGenerator::generate_boundary().is_empty());
    }

    #[test]
    fn generate_random_boundary_different_each_time() {
        assert!(
            RandomAsciiGenerator::generate_boundary() != RandomAsciiGenerator::generate_boundary()
        );
    }
}
