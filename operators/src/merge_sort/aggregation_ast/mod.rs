pub mod prove;
pub mod provider;
pub mod trace;
pub mod verify;

#[cfg(test)]
pub mod test;

#[derive(Clone, Debug)]
pub struct DataFrameRow<T: Clone> {
    start: Vec<T>,
    end: Vec<T>,
    commitment: Vec<T>,
}
