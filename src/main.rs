use field_element::FieldElement;
use rand::distributions::*;
use ndarray::*;
use ndarray_rand::RandomExt;
use ndarray_linalg::solve::Inverse;

fn main() {
    let mut sum_encoded_sample:Vec<FieldElement<i32>> = Vec::new();

    // ランダムなサンプルデータを生成
    let sample: Array1<FieldElement<i32>> = Array::random(5, Uniform::new(0, 255)).mapv(|x| FieldElement::new(x, 256));
    println!("Original sample: {:?}", sample);

    // ランダムな行列を生成
    let random_matrix: Array2<i32> = Array::random((1, 5), Uniform::new(0, 255)).mapv(|x| x);
    println!("Random matrix: {:?}", random_matrix);
    // 符号化
    let encoded_sample = &sample*&random_matrix;
    println!("Encoded sample: {:?}", encoded_sample);

     // 合計値を計算
    let sum: FieldElement<i32> = encoded_sample.iter().fold(FieldElement::new(0, 256), |acc, x| acc + x.clone());
    println!("Sum of encoded sample: {:?}", sum);
    sum_encoded_sample.push(sum);
}
