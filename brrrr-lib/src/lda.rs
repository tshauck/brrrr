// (c) Copyright 2020 Trent Hauck
// All Rights Reserved


use nalgebra::{DMatrix, Matrix2};
use rand_distr::{Distribution, Gamma};
use statrs::function::gamma::digamma;

use log::info;

pub struct OnlineLDA {
    num_topics: usize,
    num_documents: usize,
    vocab_size: usize,
    exp_e_log_beta: DMatrix<f32>,
}

fn dirichlet_expectation(alpha: DMatrix<f32>) -> DMatrix<f32> {
    let (rows, columns) = alpha.shape();
    let mut new_gamma = DMatrix::<f32>::zeros(rows, columns);

    for i in 0..rows {
        let row_i = alpha.row(i);

        // let row_i_psi = row_i.map(|f| digamma(f as f64));
        let psi_sum = digamma(row_i.sum() as f64) as f32;

        info!("For row {} got psi sum {}.", row_i, psi_sum);

        for j in 0..columns {
            new_gamma[(i, j)] = (digamma(alpha[(i, j)] as f64) as f32) - psi_sum;
            println!("{:?} {:?}", new_gamma[(i, j)], psi_sum)
        }
    }
    println!("new gamma {:?}", new_gamma);
    new_gamma
}

impl OnlineLDA {
    fn update_lambda(&self, word_ids: Vec<Vec<i32>>, word_counts: Vec<i32>) {
        self.do_e_step(word_ids, word_counts);
    }

    fn do_e_step(&self, word_ids: Vec<Vec<i32>>, word_counts: Vec<i32>) {
        let batch_document_size = word_ids.len();

        let randr = rand::thread_rng();

        // TODO: This might be a bad initialization.
        let gamma_dist = Gamma::new(100.0, 1.0 / 100.0).unwrap();

        // Create the matrix for gamma, and fill it with random samples.
        let mut gamma = DMatrix::<f32>::zeros(batch_document_size, self.num_topics);
        for i in 0..batch_document_size {
            for j in 0..self.num_topics {
                gamma[(i, j)] = gamma_dist.sample(&mut rand::thread_rng());
            }
        }

        let mut e_log_theta = DMatrix::<f32>::zeros(batch_document_size, self.num_topics);
        // let mut e_log_theta = dirichlet_expectation(gamma);

        // For each document in the document batch.
        for document_i in 0..batch_document_size {
            // Information about this document, its word content and counts.
            let ids = word_ids.get(document_i).unwrap();
            let counts = word_counts.get(document_i).unwrap();

            let gamma_d = gamma.row(document_i);
            let e_log_theta_d = e_log_theta.row(document_i);
            let e_log_beta_d = self.exp_e_log_beta.row(document_i);

            let phinorm = e_log_theta_d.dot(&e_log_beta_d);

            // https://github.com/blei-lab/onlineldavb/blob/dee5dcf9492d2b2870ba5c1fc14ac41cbf83596c/onlineldavb.py#L156-L170
            for it in 0..100 {
                let lastgamma = gamma_d;
            }
        }
    }

    fn new(num_topics: usize, num_documents: usize, vocab_size: usize) -> OnlineLDA {
        let exp_e_log_beta = DMatrix::<f32>::zeros(num_topics, vocab_size);
        OnlineLDA {
            num_topics: num_topics,
            vocab_size: vocab_size,
            num_documents: num_documents,
            exp_e_log_beta: exp_e_log_beta,
        }
    }
}

#[cfg(test)]
mod tests {
    use log::error;

    use super::*;

    #[ctor::ctor]
    fn init() {
        std::env::set_var("RUST_LOG", "trace");
        env_logger::init();
        info!("HI");
    }

    #[test]
    fn test_update_lambda() {
        let old_lda = OnlineLDA::new(10, 10, 10);

        let word_ids = vec![vec![1, 2, 3], vec![1, 2], vec![1, 2, 2]];
        let word_counts = word_ids.iter().map(|f| f.len() as i32).collect();

        old_lda.update_lambda(word_ids, word_counts);
    }

    #[test]
    fn test_dirichlet_expectation() {
        let (rows, columns) = (2, 3);
        let dm = DMatrix::from_row_slice(rows, columns, &[1.0, 2.0, 3.0, 1.0, 3.0, 2.0]);

        let actual = dirichlet_expectation(dm);
        let expected = DMatrix::<f32>::from_row_slice(
            rows,
            columns,
            &[
                -2.28333333, -1.28333333, - 0.78333333,
                -2.28333333, -0.78333333, - 1.28333333,
            ],
        );

        let (actual_rows, actual_columns) = actual.shape();
        assert_eq!(actual_rows, rows);
        assert_eq!(actual_columns, columns);

        let eq = actual.eq(&expected);
        assert!(eq);
    }

    #[test]
    fn test_online_lda_init() {
        let old_lda = OnlineLDA::new(10, 10, 50);

        assert_eq!(old_lda.num_topics, 10);
        assert_eq!(old_lda.num_documents, 10);
        assert_eq!(old_lda.vocab_size, 50);
    }
}
