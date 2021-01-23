// (c) Copyright 2020 Trent Hauck
// All Rights Reserved

use nalgebra::DMatrix;
use rand::prelude::ThreadRng;
use rand_distr::{Distribution, Gamma};
use statrs::function::gamma::digamma;

use log::info;

use crate::ldalib::{dirichlet_expectation, get_gamma_random_matrix};

/// LDAState is an object that holds the necessary statistics in order to
/// make streaming updates to the inference model.
pub struct LDAState {
    /// The prior probabilities for the terms.
    eta: DMatrix<f64>,

    /// The shape of the sufficient statistics.
    sufficient_stats: DMatrix<f64>,

    /// Num docs
    num_docs: usize,
}

impl Clone for LDAState {
    fn clone(&self) -> LDAState {
        let eta_shape = self.eta.shape();

        let mut eta_zeros = DMatrix::<f64>::zeros(eta_shape.0, eta_shape.1);
        eta_zeros.copy_from(&self.eta);

        let sstats_shape = self.sufficient_stats.shape();
        let mut sstats = DMatrix::<f64>::zeros(sstats_shape.0, sstats_shape.1);

        sstats.copy_from(&self.sufficient_stats);

        LDAState {
            eta: eta_zeros,
            sufficient_stats: sstats,
            num_docs: self.num_docs,
        }
    }
}

impl LDAState {
    /// Set up LDA State.
    fn new(eta: DMatrix<f64>, sstats_shape: (usize, usize)) -> LDAState {
        let sufficient_stats = DMatrix::<f64>::zeros(sstats_shape.0, sstats_shape.1);

        LDAState {
            eta: eta,
            sufficient_stats: sufficient_stats,
            num_docs: 0,
        }
    }

    fn with_sufficient_stats(&mut self, sufficient_stats: &DMatrix<f64>) -> &mut LDAState {
        self.sufficient_stats.copy_from(sufficient_stats);
        self
    }

    fn reset(&mut self) {
        let current_shape = self.sufficient_stats.shape();

        self.sufficient_stats = DMatrix::<f64>::zeros(current_shape.0, current_shape.1);
        self.num_docs = 0;
    }

    fn merge(&mut self, other: &LDAState) {
        self.sufficient_stats = &self.sufficient_stats + &other.sufficient_stats;
        self.num_docs = self.num_docs + other.num_docs;
    }
}

/// A struct for holding parameters for training of an online LDA model.
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

    /// The hyper parameter, alpha.
    alpha: f64,
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
            alpha: 1.0,
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

            let phinorm = e_log_theta_d.dot(&e_log_beta_d) + 1e-100;

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
    fn test_lda_state_operations() {
        // Test that LDAState and its operations behave as expected.
        let eta = DMatrix::<f64>::zeros(10, 10);

        let mut state = LDAState::new(eta, (5, 5));

        let batch_document_size = 10;
        let num_topics = 10;

        let gamma = get_gamma_random_matrix(batch_document_size, num_topics);
        let expected_sufficient_stats = DMatrix::<f64>::zeros(batch_document_size, num_topics);

        state.sufficient_stats = gamma;

        let not_eq_zero = expected_sufficient_stats.eq(&state.sufficient_stats);
        assert!(!not_eq_zero);

        state.reset();

        let eq_zero = expected_sufficient_stats.eq(&state.sufficient_stats);
        assert!(eq_zero);

        let merge_gamma = get_gamma_random_matrix(batch_document_size, num_topics);
        let merge_eta = DMatrix::<f64>::zeros(10, 10);

        let mut merge_state = LDAState::new(merge_eta, merge_gamma.shape());
        merge_state.with_sufficient_stats(&merge_gamma);

        state.merge(&merge_state);

        // The state should be equal to gamma, because the original state is reset.
        let eq_merge_gamma = merge_state.sufficient_stats.eq(&merge_gamma);
        assert!(eq_merge_gamma);
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
