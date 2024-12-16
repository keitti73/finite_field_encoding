use field_element::FieldElement;
use rand::distributions::*;
use ndarray::prelude::*;
use ndarray_rand::RandomExt;
mod encoder;
mod decoder;

use std::fmt::Debug;
use std::ops::Sub;

struct Sample<T>{
    value: Vec<T>,
    prime: T,
}
impl<T> Sample<T>
where
    T: Clone + Debug + PartialOrd + From<u32> + Sub<Output = T> + Into<i128>,
{
    fn get_value(&self) -> Vec<FieldElement<i128>> {
        let prime: i128 = self.prime.clone().into();
        self.value
            .clone()
            .iter()
            .map(|x| FieldElement::new((*x).clone().into(), prime))
            .collect()
    }
    fn get_prime(&self) -> i128 {
        self.prime.clone().into()
    }

    fn random_sample() -> Sample<u32> {
        let prime: u32 = 2_u32.pow(31) - 1;
        let shape:u8 = rand::random();
        let value = Array::random(shape as usize , Uniform::new(0,prime-1)).to_vec();
        Sample { value, prime }
    }
}


fn main() {
    let sample_data: Sample<u32> = Sample::<u32>::random_sample();

    let prime: i128 = sample_data.get_prime();
    let sample: Array1<FieldElement<i128>> = sample_data.get_value().into();

    let mut sum_matrix: Vec<FieldElement<i128>> = Vec::new();
    let mut random_matrix_tmp: Vec<Vec<FieldElement<i128>>> = Vec::new();
    //println!("Original sample: {:?}", sample);

    for _ in 0..sample.len() {
        let encoded = encoder::encoding(&sample, prime);
        random_matrix_tmp.push(encoded.get_random_matrix().to_vec());
        sum_matrix.push(encoded.get_value());
    }

    let x_vec = decoder::decoding(&sum_matrix, &random_matrix_tmp, &sample.len());

    println!("{:?}", sample==x_vec);
    println!("{:?}", sample.len());
}


