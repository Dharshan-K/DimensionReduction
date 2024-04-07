use std::vec::{self, Vec};

pub struct PCA{ 
    input_matrix: Vec<Vec<f64>>,
}

impl PCA{
    pub fn new(matrix: Vec<Vec<f64>>)->Self{
        PCA{ input_matrix: matrix }
    }
    pub fn calculate_mean(&mut self)-> Option<Vec<Vec<f64>>> {
        let length = self.input_matrix[0].len();
        let mut matrix_mean = vec![0.0; 5];

        for i in 0..length{
            matrix_mean[i] = self.input_matrix.iter().map(|row| row[i]).sum::<f64>() / 3.0;
        }  

        if(length==1){
            return Some(vec![matrix_mean])
        }else {
            return Some(vec![matrix_mean])
        }
    }             
}
    
