// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

use nalgebra::{DMatrix};
use rand_distr::{Distribution, Gamma};
use statrs::function::gamma::digamma;

pub fn dirichlet_expectation(alpha: &DMatrix<f64>) -> DMatrix<f64> {
    let (rows, columns) = alpha.shape();
    let mut new_gamma = DMatrix::<f64>::zeros(rows, columns);

    for i in 0..rows {
        let row_i = alpha.row(i);

        // let row_i_psi = row_i.map(|f| digamma(f as f64));
        let psi_sum = digamma(row_i.sum() as f64) as f64;

        for j in 0..columns {
            new_gamma[(i, j)] = (digamma(alpha[(i, j)] as f64) as f64) - psi_sum;
        }
    }
    new_gamma
}

pub fn get_gamma_random_matrix(rows: usize, columns: usize) -> DMatrix<f64> {
    let mut gamma = DMatrix::<f64>::zeros(rows, columns);

    // TODO: This might be a bad initialization.
    let gamma_dist = Gamma::new(100.0, 1.0 / 100.0).unwrap();

    for i in 0..rows {
        for j in 0..columns {
            gamma[(i, j)] = gamma_dist.sample(&mut rand::thread_rng());
        }
    }

    gamma
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirichlet_expectation() {
        let (rows, columns) = (2, 3);
        let dm = DMatrix::from_row_slice(rows, columns, &[1.0, 2.0, 3.0, 1.0, 3.0, 2.0]);

        let actual = dirichlet_expectation(&dm);
        let expected = DMatrix::<f64>::from_row_slice(
            rows,
            columns,
            &[
                -2.28333333,
                -1.28333333,
                -0.78333333,
                -2.28333333,
                -0.78333333,
                -1.28333333,
            ],
        );

        let (actual_rows, actual_columns) = actual.shape();
        assert_eq!(actual_rows, rows);
        assert_eq!(actual_columns, columns);

        let relative_eq = actual.relative_eq(&expected, 0.01, 1.0);
        assert!(relative_eq);
    }
}
