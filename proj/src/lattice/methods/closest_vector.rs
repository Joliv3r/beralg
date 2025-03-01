use crate::lattice::{methods::closest_vector, Lattice};
use itertools::Itertools;
use ndarray::{Array, Array1, ArrayBase};

use super::get_length_of_vector;

impl Lattice {
    pub fn babai_nearest_plane(&self, vector: &Array1<f64>) -> Result<Array1<f64>, String> {
        let rows = self.get_length_of_basis_vectors();
        if rows != vector.len() {
            return Err("Vector size not compatible with lattice".to_string())
        }

        let dim = self.columns();
        if dim == 0 {
            return Err("Lattice is empty.".to_string())
        }
        let mut w = vector.clone();
        let mut y = Array1::zeros(rows);

        for i in (0..dim).into_iter().rev() {
            let gs_i = self.get_gram_schmidt_basis_vector(i).expect("Vector should exist.");
            let b_i = self.get_basis_vector(i).expect("Vector should exist.");
            let l_i = &w.dot(&gs_i)/gs_i.dot(&gs_i);
            let l_i_rounded = l_i.round();

            y = y + &b_i*l_i_rounded;

            w = w - gs_i*(l_i - l_i_rounded) - b_i*l_i_rounded;
        }

        Ok(y)
    }

    pub fn closest_vector_by_enumeration(&self, vector: &Array1<f64>) -> Result<(Array1<f64>, Vec<(f64, f64)>), String> {
        if self.columns() == 0 {
            return Err("Lattice is empty.".to_string())
        }

        let y = self.write_vector_with_gram_schmidt_vectors(vector);
        println!("We have writting vector as {}", y);
        let mut closest_vector = self.get_basis_vector(0).expect("Should exist.");
        let mut shortest_distance = get_length_of_vector(&(vector-&closest_vector));

        let mut checked_points = vec![(closest_vector[0], closest_vector[1])];

        let (lower_bound, upper_bound) = self.get_cvp_enumeration_bounds(&Vec::<i64>::new(), 0, shortest_distance, &y);
        println!("Checking in {}..{}", lower_bound, upper_bound);

        for x_n in lower_bound..=upper_bound {
            let combination = vec![x_n];
            let (candidate_vector, candidate_distance) = self.closest_vector_enumeration_step(vector, 1, &combination, &closest_vector, shortest_distance, &y, &mut checked_points);
            if candidate_distance < shortest_distance {
                closest_vector = candidate_vector;
                shortest_distance = candidate_distance;
            }
        }

        Ok((closest_vector, checked_points))
    }

    fn closest_vector_enumeration_step(&self, vector: &Array1<f64>, depth: usize, combination: &Vec<i64>, current_closest_vector: &Array1<f64>, current_shortest_distance: f64, y: &Array1<f64>, checked_points: &mut Vec<(f64, f64)>) -> (Array1<f64>, f64) {
        if depth == self.columns() {
            let indexes = &combination.iter().rev().cloned().collect_vec();
            println!("We have indexes {:?}", indexes);
            let current_vector = self.get_lattice_point(indexes).expect("Should exist.");
            let current_distance = get_length_of_vector(&(vector-&current_vector));
            // println!("Found current distance: {}, with {}", current_distance, current_vector);
            checked_points.push((current_vector[0], current_vector[1]));
            return (current_vector, current_distance)
        }
        
        let mut closest_vector = current_closest_vector.clone();
        let mut shortest_distance = current_shortest_distance;
        let (lower_bound, upper_bound) = self.get_cvp_enumeration_bounds(combination, depth, shortest_distance, y);

        // println!("Checking {}, with {} .. {}", combination[0], lower_bound, upper_bound);

        for i in lower_bound..=upper_bound {
            let mut new_combination = combination.clone();
            new_combination.push(i);
            let (candidate_vector, candidate_distance) = self.closest_vector_enumeration_step(vector, depth+1, &new_combination, &closest_vector, shortest_distance, y, checked_points);
            if candidate_distance < shortest_distance {
                closest_vector = candidate_vector;
                shortest_distance = candidate_distance;
                // println!("New shortest distance found: {}, with {}", shortest_distance, closest_vector);
            }            
        }

        (closest_vector, shortest_distance)
    }

    #[allow(non_snake_case)]
    fn get_cvp_enumeration_bounds(&self, combination: &Vec<i64>, basis_number: usize, A: f64, y: &Array1<f64>) -> (i64, i64) {
        println!("Checking bounds with following information: {:?}, {}, {}", combination, basis_number, A);
        let mut sum = 0.;
        let mut N_i = 0.;
        let start = self.columns()-basis_number;
        let end = self.columns();
        let i = self.columns()-basis_number-1;
        // let z = self.write_vector_with_gram_schmidt_vectors_from_reversed_basis_representation(combination);
        for (x_j,  j) in combination.iter().zip((start..end).into_iter().rev()) {
            let B_j = self.get_gram_schmidt_length(j);
            let mu_ji = self.get_mu(j, i);
            sum += ((self.get_gram_schmidt_coefficient(&combination) - y.get(j).expect("Should exist.")).powf(2.) as f64)*B_j;
            N_i += mu_ji * (*x_j as f64);
        }
        let M_i = ((A - sum).abs()/self.get_gram_schmidt_length(i)).sqrt();
        let y_i = y.get(i).expect("Should exist.");
        // println!("We have y_i = {}", y_i);

        (
            (y_i - M_i - N_i).ceil() as i64,
            (y_i + M_i - N_i).floor() as i64
        )
    }

    // This help function is only for get_cvp_enumeration_bounds(...), and will therefore be
    // written to work with this, and not for general purpose.
    pub fn get_gram_schmidt_coefficient(&self, combination: &Vec<i64>) -> f64 {
        // let num_of_zeros = self.columns()-combination.len();
        // let comb = combination.iter().copied().chain(vec![0; num_of_zeros].iter().copied()).rev().collect();
        // let vector = self.get_lattice_point(&comb).expect("Combination should be valid.");

        let i = self.columns() - combination.len() - 1;
        let mut z_i = 0.;
        for (j, x_j) in combination.iter().enumerate() {
            if j == combination.len()-1 {
                z_i += *x_j as f64;
            }
            z_i += (*x_j as f64)*self.get_mu(self.columns()-j-1, i);
        }

        z_i
    }
}
