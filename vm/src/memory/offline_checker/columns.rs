use afs_primitives::offline_checker::columns::OfflineCheckerCols;

use super::MemoryOfflineChecker;

#[derive(Debug, derive_new::new)]
pub struct MemoryOfflineCheckerCols<T> {
    pub offline_checker_cols: OfflineCheckerCols<T>,
    pub is_final_access: T,
}

impl<T> MemoryOfflineCheckerCols<T>
where
    T: Clone,
{
    pub fn flatten(&self) -> Vec<T> {
        let mut flattened = self.offline_checker_cols.flatten();
        flattened.push(self.is_final_access.clone());

        flattened
    }

    pub fn from_slice(slc: &[T], oc: &MemoryOfflineChecker) -> Self {
        assert!(slc.len() == oc.air_width());

        let offline_checker_cols_width = oc.offline_checker.air_width();
        let offline_checker_cols =
            OfflineCheckerCols::from_slice(&slc[..offline_checker_cols_width], &oc.offline_checker);

        Self {
            offline_checker_cols,
            is_final_access: slc[offline_checker_cols_width].clone(),
        }
    }

    pub fn width(oc: &MemoryOfflineChecker) -> usize {
        oc.offline_checker.air_width() + 1
    }
}
