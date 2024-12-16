use field_element::FieldElement;
use rand::distributions::*;
use ndarray::prelude::*;
use ndarray_rand::RandomExt;

struct Encoded {
    random_matrix: Array1<FieldElement<i128>>,
    value: FieldElement<i128>,
}

fn main() {
    let prime = (2_u32.pow(31) - 1) as i128;
    let mut sum_matrix: Vec<FieldElement<i128>> = Vec::new();
    let mut random_matrix_1: Vec<Vec<FieldElement<i128>>> = Vec::new();
    // ランダムなサンプルデータを生成
    let sample: Array1<FieldElement<i128>> = Array::random(5, Uniform::new(0, prime - 1)).mapv(|x| FieldElement::new(x, prime));
    //println!("Original sample: {:?}", sample);

    for _ in 0..5 {
        //let random_matrix: Array1<FieldElement<i128>> = Array::random(5, Uniform::new(0, 255)).mapv(|x| FieldElement::new(x, prime));
        //random_matrix_1.push(random_matrix.to_vec());

        // 符号化
        //let encoded_sample = &sample * &random_matrix;

        // 合計値を計算するクロージャ
        //let sum_closure = |arr: &Array1<FieldElement<i128>>| -> FieldElement<i128> {
        //    arr.iter().fold(FieldElement::new(0, prime), |acc, x| acc + x.clone())
        //};

        // 合計値を計算
        //let sum = sum_closure(&encoded_sample);
        //sum_matrix.push(sum)
        let encoded = encoding(&sample, prime);
        random_matrix_1.push(encoded.random_matrix.to_vec());
        sum_matrix.push(encoded.value);
    }

    let sum_matrix = Array::from(sum_matrix).to_owned();
    let random_matrix: Array2<FieldElement<i128>> = Array2::from_shape_vec((5, 5), random_matrix_1.iter().flatten().cloned().collect()).unwrap();
    //println!("Random matrix: {:?}", random_matrix);
    let x_vec = gaussian_elimination(&random_matrix, &sum_matrix);
    //println!("Decoded sample: {:?}", x_vec);

    println!("{:?}", sample==x_vec);
}

fn gaussian_elimination(a: &Array2<FieldElement<i128>>, b: &Array1<FieldElement<i128>>) -> Array1<FieldElement<i128>> {
    let n = a.nrows();
    let mut a = a.clone();
    let mut b = b.clone();

    for i in 0..n {
        let mut max_row = i;
        for k in (i + 1)..n {
            if a[[k, i]].num.abs() > a[[max_row, i]].num.abs() {
                max_row = k;
            }
        }
        // 手動で行を入れ替える
        for j in 0..a.ncols() {
            a.swap([i, j], [max_row, j]);
        }
        b.swap(i, max_row);

        for k in (i + 1)..n {
            let factor = a[[k, i]].clone() / a[[i, i]].clone();
            for j in i..n {
                a[[k, j]] = a[[k, j]].clone() - factor.clone() * a[[i, j]].clone();
            }
            b[k] = b[k].clone() - factor * b[i].clone();
        }
    }

    let mut x: Array1<FieldElement<i128>> = Array1::from_elem(n, FieldElement::new(0, a[[0, 0]].prime));
    for i in (0..n).rev() {
        let mut sum = FieldElement::new(0, a[[i, i]].prime);
        for j in (i + 1)..n {
            sum = sum + a[[i, j]].clone() * x[j].clone();
        }
        x[i] = (b[i].clone() - sum) / a[[i, i]].clone();
    }

    x
}

fn encoding(sample: &Array1<FieldElement<i128>>, prime: i128) -> Encoded {
    let prime = prime.clone();
    let sample = sample.clone();
    let random_matrix: Array1<FieldElement<i128>> = Array::random(5, Uniform::new(0, 255)).mapv(|x| FieldElement::new(x, prime));
    let encoded_sample = &sample * &random_matrix;
    let sum_closure = |arr: &Array1<FieldElement<i128>>| -> FieldElement<i128> {
        arr.iter().fold(FieldElement::new(0, prime), |acc, x| acc + x.clone())
    };
    let sum = sum_closure(&encoded_sample);
    return Encoded{random_matrix, value:sum};
}