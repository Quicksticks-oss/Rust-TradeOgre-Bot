use ndarray::{Array1, ArrayView1};
use std::collections::HashMap;

pub struct VectorStore {
    vector_data: HashMap<String, Array1<f64>>,
    vector_index: HashMap<String, HashMap<String, f64>>,
}

impl VectorStore {
    pub fn new() -> Self {
        Self {
            vector_data: HashMap::new(),
            vector_index: HashMap::new(),
        }
    }

    pub fn add_vector(&mut self, vector_id: String, vector: Array1<f64>) {
        self.vector_data.insert(vector_id.clone(), vector.clone());
        self.update_index(vector_id, vector);
    }

    pub fn get_vector(&self, vector_id: &str) -> Option<&Array1<f64>> {
        self.vector_data.get(vector_id)
    }

    fn update_index(&mut self, vector_id: String, vector: Array1<f64>) {
        for (existing_id, existing_vector) in self.vector_data.iter() {
            let similarity = cosine_similarity(vector.view(), existing_vector.view());
            self.vector_index
                .entry(existing_id.clone())
                .or_insert_with(HashMap::new)
                .insert(vector_id.clone(), similarity);
        }
    }

    pub fn find_similar_vectors(
        &self,
        query_vector: Array1<f64>,
        num_results: usize,
    ) -> Vec<(String, f64)> {
        let mut results = Vec::new();
        for (vector_id, vector) in self.vector_data.iter() {
            let similarity = cosine_similarity(query_vector.view(), vector.view());
            results.push((vector_id.clone(), similarity));
        }

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        results.truncate(num_results);
        results
    }
}

fn cosine_similarity(a: ArrayView1<f64>, b: ArrayView1<f64>) -> f64 {
    let norm_a = a.map(|x| x.powi(2)).sum().sqrt();
    let norm_b = b.map(|x| x.powi(2)).sum().sqrt();
    a.dot(&b) / (norm_a * norm_b)
}
