use field_element::FieldElement;

#[derive(Debug)]
#[allow(dead_code)]
pub struct TransmissionData {
    pub data: u64,
    pub random_matrix: Vec<u8>,
    pub prime: i128,
}
impl TransmissionData {
    //pub fn to_encoded(&self) -> Encoded {
    //    let random_matrix: Vec<FieldElement<i128>> = self.random_matrix.iter().map(|x| FieldElement::new(*x as i128, self.prime)).collect();
    //    let value = FieldElement::new(self.data as i128, self.prime);
    //    return Encoded { random_matrix: random_matrix.into(), value };
    //}
    pub fn to_value(&self) -> FieldElement<i128> {
        let value = FieldElement::new(self.data as i128, self.prime);
        value
    }
    pub fn get_prime(&self) -> i128 {
        self.prime
    }
    pub fn get_random_matrix(&self) -> Vec<FieldElement<i128>> {
        self.random_matrix.iter().map(|x| FieldElement::new(*x as i128, self.prime)).collect()
    }
}