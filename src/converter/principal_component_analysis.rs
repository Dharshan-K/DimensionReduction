use std::vec::{self, Vec};
use ndarray::{Array2, Axis, Zip}; 
use ndarray::prelude::{ArrayBase,Dim};

pub struct PCA{ 
    input_matrix: Array2<f64>,
}

impl PCA{
    pub fn new(matrix: Array2<f64>)->Self{
        PCA{ input_matrix: matrix }
    }
    pub fn calculate_mean(&mut self) {        
        let mut matrix_mean: ArrayBase<ndarray::OwnedRepr<f64>, Dim<[usize; 1]>> = self.input_matrix.mean_axis(Axis(0)).unwrap();

        // let mut covariance_matrix = Zip::from(&self.input_matrix).and(&matrix_mean).map_collect(|&x,&y| x - y);
        
        for i in 0..self.input_matrix.len_of(Axis(0)){
            print!("{}", matrix_mean[i]);
        }  

        

        // let mut covarince_matrix = self.input_matrix - matrix_mean;     

        // if(length==1){
        //     return Some(vec![matrix_mean])
        // }else {
        //     return Some(vec![matrix_mean])
        // }
    }     

       
}
    
