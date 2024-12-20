pub use field_element::FieldElement;
use rand::distributions::*;
pub use ndarray::prelude::*;
use ndarray_rand::RandomExt;
use crate::transmission::TransmissionData;
pub struct Encoded {
    random_matrix: Array1<FieldElement<i128>>,
    value: FieldElement<i128>,
}
impl Encoded {
    fn new(random_matrix: Array1<FieldElement<i128>>, sample:Array1<FieldElement<i128>>,prime: i128) -> Self {
        let encoded_sample = &sample * &random_matrix;
        let sum_closure = |arr: &Array1<FieldElement<i128>>| -> FieldElement<i128> {
            arr.iter().fold(FieldElement::new(0, prime), |acc, x| acc + x.clone())
        };
        let sum = sum_closure(&encoded_sample);
        return Encoded { random_matrix:random_matrix, value: sum, };
    }
    pub fn get_random_matrix(&self) -> Array1<FieldElement<i128>> {
        return self.random_matrix.clone();
    }
    pub fn get_value(&self) -> FieldElement<i128> {
        return self.value.clone();
    }
    pub fn to_transmission_data(&self) -> TransmissionData {
        let random_matrix = self.random_matrix.iter().map(|x| x.num as u8).collect();
        let data = self.value.num as u64;
        return TransmissionData { data, random_matrix , prime: self.value.prime as u64};
    }
}

pub fn encoding(sample: &Array1<FieldElement<i128>>, prime: i128) -> Encoded {
    let prime = prime.clone();
    let sample_sahpe = sample.len();
    let sample = sample.clone();
    let random_matrix: Array1<FieldElement<i128>> = Array::random(sample_sahpe, Uniform::new(0, 255)).mapv(|x| FieldElement::new(x, prime));
    return Encoded::new(random_matrix, sample, prime);
}

