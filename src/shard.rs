// Copyright 2023 Martin Pool

//! Sharding parameters.

use std::str::FromStr;

use anyhow::{anyhow, ensure, Context, Error};

/// Select mutants for a particular shard of the total list.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Shard {
    /// Index modulo n.
    pub k: usize,
    /// Modulus of sharding.
    pub n: usize,
}

impl Shard {
    /// Select the mutants that should be run for this shard.
    pub fn select<M, I: IntoIterator<Item = M>>(&self, mutants: I) -> Vec<M> {
        mutants
            .into_iter()
            .enumerate()
            .filter_map(|(i, m)| if i % self.n == self.k { Some(m) } else { None })
            .collect()
    }
}

impl FromStr for Shard {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once('/').ok_or(anyhow!("shard must be k/n"))?;
        let k = parts.0.parse().context("shard k")?;
        let n = parts.1.parse().context("shard n")?;
        ensure!(k < n, "shard k must be less than n"); // implies n>0
        Ok(Shard { k, n })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shard_from_str_valid_input() {
        let shard = Shard::from_str("2/5").unwrap();
        assert_eq!(shard.k, 2);
        assert_eq!(shard.n, 5);
        assert_eq!(shard, Shard { k: 2, n: 5 });
    }

    #[test]
    fn shard_from_str_invalid_input() {
        assert_eq!(
            Shard::from_str("").unwrap_err().to_string(),
            "shard must be k/n"
        );

        assert_eq!(
            Shard::from_str("2").unwrap_err().to_string(),
            "shard must be k/n"
        );

        assert_eq!(
            Shard::from_str("2/0").unwrap_err().to_string(),
            "shard k must be less than n"
        );

        assert_eq!(
            Shard::from_str("5/2").unwrap_err().to_string(),
            "shard k must be less than n"
        );
    }

    #[test]
    fn shard_select() {
        assert_eq!(
            Shard::from_str("1/4").unwrap().select(0..10).as_slice(),
            &[1, 5, 9]
        );
    }
}
