pub mod converter;
use ndarray::Array2;

fn main() {
    println!("Hello, world!");
    let input_matrix: Array2<f64> = Array2::from_shape_vec((3, 3), vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0
    ]).unwrap();
    let mut pca: converter::principal_component_analysis::PCA = crate::converter::principal_component_analysis::PCA::new(input_matrix);
    pca.calculate_mean();
} 
