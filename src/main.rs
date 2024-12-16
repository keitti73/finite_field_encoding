use field_element::FieldElement;
//use rand::distributions::*;
use ndarray::prelude::*;
//use ndarray_rand::RandomExt;
mod encoder;
mod decoder;

use std::fmt::Debug;
use std::ops::Sub;

use std::fs::File;
use std::io::{Read, Write};

struct Sample<T,U>{
    value: Vec<T>,
    prime: U,
}
impl<T,U> Sample<T,U>
where
    T: Clone + Debug + PartialOrd + From<u32> + Sub<Output = T> + Into<i128>,
    U: Clone + Debug + Into<i128>,
{
    fn to_value(&self) -> Vec<FieldElement<i128>> {
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

    //fn random_sample() -> Sample<u32,u32> {
    //    let prime: u32 = 2_u32.pow(31) - 1;
    //    let shape:u8 = rand::random();
    //    let value = Array::random(shape as usize , Uniform::new(0,prime-1)).to_vec();
    //    Sample { value, prime }
    //}
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("./src/main.rs")?;
    let mut buf: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buf)?;

    let u32_buf: Vec<u32> = buf.chunks_exact(4)
        .map(|chunk| {
            let mut array = [0u8; 4];
            array.copy_from_slice(chunk);
            u32::from_le_bytes(array)
        })
        .collect();

    //println!("{:?}", u32_buf);

    let sample_data: Sample<u32,u32> = Sample { value: u32_buf.clone(), prime:  2_u32.pow(31) - 1 };

    let prime: i128 = sample_data.get_prime();
    let sample: Array1<FieldElement<i128>> = sample_data.to_value().into();

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


    let x_vec_u32: Vec<u32> = x_vec.into_iter().map(|x| x.num as u32).collect();
    println!("{}", u32_buf == x_vec_u32);

    let mut file = File::create("out.rs")?;
    for num in x_vec_u32 {
        file.write_all(&num.to_le_bytes())?;
    };
    buf.flush()?;
    Ok(())
}
