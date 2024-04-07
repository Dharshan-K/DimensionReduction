pub mod converter;

fn main() {
    println!("Hello, world!");
    let input_matrix = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0]
    ];


    let mut pca: converter::principal_component_analysis::PCA = crate::converter::principal_component_analysis::PCA::new(input_matrix);
    pca.calculate_mean();
} 
