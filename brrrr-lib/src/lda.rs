// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

use nalgebra::DMatrix;
use rand::prelude::ThreadRng;
use rand_distr::{Distribution, Gamma};
use statrs::function::gamma::digamma;

use log::info;

use crate::ldalib::{dirichlet_expectation, get_gamma_random_matrix};

/// A struct for holding the state and training of an online LDA model.
pub struct OnlineLDA {

    /// The number of topics used during training, K.
    num_topics: usize,

    ///  The overall number of documents in the corpus, D.
    num_documents: usize,

    /// The vocabulary size of the data to be fit, W.
    vocab_size: usize,

    /// The lambda parameter of the variational distribution, q(beta|lambda).
    lambda: DMatrix<f64>,

    /// The exectation of the log of beta, given lambda.
    e_log_beta: DMatrix<f64>,

    /// The exponentiated expectation of beta, given lambda.
    exp_e_log_beta: DMatrix<f64>,
}

impl OnlineLDA {
    fn update_lambda(&self, word_ids: Vec<Vec<i32>>, word_counts: Vec<i32>) {
        self.do_e_step(word_ids, word_counts);
    }

    fn new(num_topics: usize, num_documents: usize, vocab_size: usize) -> OnlineLDA {
        let lambda = get_gamma_random_matrix(num_topics, vocab_size);
        let e_log_beta = dirichlet_expectation(&lambda);
        let exp_e_log_beta = e_log_beta.map(|f| f.exp());

        OnlineLDA {
            num_topics: num_topics,
            vocab_size: vocab_size,
            num_documents: num_documents,
            exp_e_log_beta: exp_e_log_beta,
            lambda: lambda,
            e_log_beta: e_log_beta,
        }
    }

    fn do_e_step(&self, word_ids: Vec<Vec<i32>>, word_counts: Vec<i32>) {
        let batch_document_size = word_ids.len();

        // Create the matrix for gamma, and fill it with random samples.
        let gamma = get_gamma_random_matrix(batch_document_size, self.num_topics);
        let e_log_theta = dirichlet_expectation(&gamma);
        let exp_e_log_theta = e_log_theta.map(|f| f.exp());

        let sstats = DMatrix::<f64>::zeros(self.num_topics, self.vocab_size);

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ctor::ctor]
    fn init() {
        std::env::set_var("RUST_LOG", "trace");
        env_logger::init();
    }

    #[test]
    fn test_update_lambda() {
        let old_lda = OnlineLDA::new(10, 10, 10);

        let word_ids = vec![vec![1, 2, 3], vec![1, 2], vec![1, 2, 2]];
        let word_counts = word_ids.iter().map(|f| f.len() as i32).collect();

        old_lda.update_lambda(word_ids, word_counts);
    }

    #[test]
    fn test_online_lda_init() {
        let old_lda = OnlineLDA::new(10, 10, 50);

        assert_eq!(old_lda.num_topics, 10);
        assert_eq!(old_lda.num_documents, 10);
        assert_eq!(old_lda.vocab_size, 50);
    }
}
