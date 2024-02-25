use crate::Serialize;

#[derive(Debug)]
pub struct UaArray<T: Serialize> {
    dimensions: Vec<i32>,
    values: Vec<T>,
}
//we restrict the code to work with array of one dimension
impl<T> Serialize for UaArray<T>
where
    T: Serialize,
{
    fn serialize(&self) -> Vec<u8> {
        let mut result = self.dimensions.serialize();
        result.extend_from_slice(&self.values.serialize());
        result
    }
}
