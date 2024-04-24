use std::vec::{self};
use ndarray::{Array2, Axis}; 
use ndarray::prelude::{ArrayBase,Dim};
use nalgebra::{SymmetricEigen,DMatrix};

pub struct PCA{ 
    input_matrix: Array2<f64>,
}

impl PCA{
    pub fn new(matrix: Array2<f64>)->Self{
        PCA{ input_matrix: matrix }
    }
    pub fn calculate_mean(&mut self) {        
        let mut matrix_mean: ArrayBase<ndarray::OwnedRepr<f64>, Dim<[usize; 1]>> = self.input_matrix.mean_axis(Axis(0)).unwrap();
        let centered_matrix = &self.input_matrix - &matrix_mean;
        // print!("{:?}", centered_matrix);
        let input_size = self.input_matrix.shape()[0] as f64;
        let covariance_matrix = &centered_matrix.t().dot(&centered_matrix) / (&input_size - 1.0);
        let covariance_matrix_nalgebra = DMatrix::from_iterator(covariance_matrix.nrows(), covariance_matrix.ncols(), covariance_matrix.iter().copied());
        let eigen_matrix = SymmetricEigen::new(covariance_matrix_nalgebra);
        let eigen_values = eigen_matrix.eigenvalues;
        let eigen_vectors = eigen_matrix.eigenvectors;

        println!("Eigenvalues: {:?}", eigen_values);
        println!("Eigenvectors: {:?}", eigen_vectors);

        // print!("{:?}", covariance_matrix);
        
    }     

       
}
    
