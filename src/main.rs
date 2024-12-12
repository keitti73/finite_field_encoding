use field_element::FieldElement;
use rand::distributions::*;
use ndarray::prelude::*;
use ndarray_rand::RandomExt;
use ndarray_linalg::{Solve, Inverse};

fn main() {
    let prime = (2_u32.pow(31) - 1) as i128;
    let mut sum_matrix: Vec<FieldElement<i128>>= Vec::new();
    let mut random_matrix_1: Vec<Array1<FieldElement<i128>>> = Vec::new();
    // ランダムなサンプルデータを生成
    let sample: Array1<FieldElement<i128>> = Array::random(5, Uniform::new(0, 255)).mapv(|x| FieldElement::new(x, prime));
    println!("Original sample: {:?}", sample);

    for i in 0..5 {
        let random_matrix: Array1<FieldElement<i128>> = Array::random(5, Uniform::new(0, 255)).mapv(|x| FieldElement::new(x, prime));
        //println!("Random matrix: {:?}", &random_matrix);
        random_matrix_1.push(random_matrix.clone());

        // 符号化
        let encoded_sample = &sample * &random_matrix;
        //println!("Encoded sample: {:?}", encoded_sample);

        // 合計値を計算するクロージャ
        let sum_closure = |arr: &Array1<FieldElement<i128>>| -> FieldElement<i128> {
            arr.iter().fold(FieldElement::new(0, prime), |acc, x| acc + x.clone())
        };

        // 合計値を計算
        let sum = sum_closure(&encoded_sample);
        //println!("Sum of encoded sample: {:?}", &sum);
        sum_matrix.push(sum)
    }

    let sum_matrix = Array1::from_vec(sum_matrix).t() ;
    let random_matrix = Array::from_shape_vec((5, 5),random_matrix_1.into_iter().flatten().collect()).unwrap();
    println!("random_matrix_1: {:?}", random_matrix);
    let random_matrix_inv = random_matrix.inv().expect("Matrix inversion failed");
    let x_vec = random_matrix_inv.dot(&sum_matrix);

    println!("Decoded sample: {:?}", x_vec);
}
// https://github.com/shinbunbun/secp256k1-rust
// field_elementは上記のコードの一部をお借りしま・・