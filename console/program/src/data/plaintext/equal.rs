// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

impl<N: Network> Eq for Plaintext<N> {}

impl<N: Network> PartialEq for Plaintext<N> {
    /// Returns `true` if `self` and `other` are equal.
    fn eq(&self, other: &Self) -> bool {
        *self.is_equal(other)
    }
}

impl<N: Network> Equal<Self> for Plaintext<N> {
    type Output = Boolean<N>;

    /// Returns `true` if `self` and `other` are equal.
    fn is_equal(&self, other: &Self) -> Self::Output {
        match (self, other) {
            (Self::Literal(a, _), Self::Literal(b, _)) => a.is_equal(b),
            (Self::Struct(a, _), Self::Struct(b, _)) => match a.len() == b.len() {
                true => {
                    // Recursively check each member for equality.
                    let mut equal = Boolean::new(true);
                    for ((name_a, plaintext_a), (name_b, plaintext_b)) in a.iter().zip_eq(b.iter()) {
                        equal = equal & name_a.is_equal(name_b) & plaintext_a.is_equal(plaintext_b);
                    }
                    equal
                }
                false => Boolean::new(false),
            },
            (Self::List(a, _), Self::List(b, _)) => match a.len() == b.len() {
                true => {
                    // Recursively check each element for equality.
                    let mut equal = Boolean::new(true);
                    for (plaintext_a, plaintext_b) in a.iter().zip_eq(b.iter()) {
                        equal &= plaintext_a.is_equal(plaintext_b);
                    }
                    equal
                }
                false => Boolean::new(false),
            },
            (Self::Literal(..), _) | (Self::Struct(..), _) | (Self::List(..), _) => Boolean::new(false),
        }
    }

    /// Returns `true` if `self` and `other` are *not* equal.
    fn is_not_equal(&self, other: &Self) -> Self::Output {
        match (self, other) {
            (Self::Literal(a, _), Self::Literal(b, _)) => a.is_not_equal(b),
            (Self::Struct(a, _), Self::Struct(b, _)) => match a.len() == b.len() {
                true => {
                    // Recursively check each member for equality.
                    let mut not_equal = Boolean::new(false);
                    for ((name_a, plaintext_a), (name_b, plaintext_b)) in a.iter().zip_eq(b.iter()) {
                        not_equal = not_equal | name_a.is_not_equal(name_b) | plaintext_a.is_not_equal(plaintext_b);
                    }
                    not_equal
                }
                false => Boolean::new(true),
            },
            (Self::List(a, _), Self::List(b, _)) => match a.len() == b.len() {
                true => {
                    // Recursively check each element for equality.
                    let mut not_equal = Boolean::new(false);
                    for (plaintext_a, plaintext_b) in a.iter().zip_eq(b.iter()) {
                        not_equal |= plaintext_a.is_not_equal(plaintext_b);
                    }
                    not_equal
                }
                false => Boolean::new(true),
            },
            (Self::Literal(..), _) | (Self::Struct(..), _) | (Self::List(..), _) => Boolean::new(true),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_console_network::Testnet3;

    type CurrentNetwork = Testnet3;

    fn sample_plaintext() -> Plaintext<CurrentNetwork> {
        Plaintext::<CurrentNetwork>::from_str(
            r"{
    a: true,
    b: 123456789field,
    c: 0group,
    d: {
        e: true,
        f: 123456789field,
        g: 0group
    }
}",
        )
        .unwrap()
    }

    fn sample_mismatched_plaintext() -> Plaintext<CurrentNetwork> {
        Plaintext::<CurrentNetwork>::from_str(
            r"{
    a: false,
    b: 123456789field,
    c: 0group,
    d: {
        e: true,
        f: 123456789field,
        g: 0group
    }
}",
        )
        .unwrap()
    }

    fn check_is_equal() {
        // Sample the plaintext.
        let plaintext = sample_plaintext();
        let mismatched_plaintext = sample_mismatched_plaintext();

        let candidate = plaintext.is_equal(&plaintext);
        assert!(*candidate);

        let candidate = plaintext.is_equal(&mismatched_plaintext);
        assert!(!*candidate);
    }

    fn check_is_not_equal() {
        // Sample the plaintext.
        let plaintext = sample_plaintext();
        let mismatched_plaintext = sample_mismatched_plaintext();

        let candidate = plaintext.is_not_equal(&mismatched_plaintext);
        assert!(*candidate);

        let candidate = plaintext.is_not_equal(&plaintext);
        assert!(!*candidate);
    }

    #[test]
    fn test_is_equal() {
        check_is_equal()
    }

    #[test]
    fn test_is_not_equal() {
        check_is_not_equal()
    }
}
