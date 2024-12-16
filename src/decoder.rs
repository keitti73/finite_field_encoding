pub use field_element::FieldElement;
pub use rand::distributions::*;
pub use ndarray::prelude::*;
pub use ndarray_rand::RandomExt;

pub fn decoding(sum_matrix:&Vec<FieldElement<i128>>,random_matrix_tmp:&Vec<Vec<FieldElement<i128>>>) -> Array1<FieldElement<i128>> {
    let sum_matrix = sum_matrix.clone();
    let random_matrix_tmp = random_matrix_tmp.clone();

    let sum_matrix = Array::from(sum_matrix).to_owned();
    let random_matrix: Array2<FieldElement<i128>> = Array2::from_shape_vec((5, 5), random_matrix_tmp.iter().flatten().cloned().collect()).unwrap();
    //println!("Random matrix: {:?}", random_matrix);
    let x_vec = gaussian_elimination(&random_matrix, &sum_matrix);
    //println!("Decoded sample: {:?}", x_vec);
    return x_vec;
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