use afs_derive::AlignedBorrow;

#[derive(Default, AlignedBorrow)]
pub struct IsLessThanIOCols<T> {
    pub x: T,
    pub y: T,
    pub less_than: T,
}

pub struct IsLessThanAuxCols<T> {
    pub lower: T,
    // lower_decomp consists of lower decomposed into limbs of size decomp where we also shift
    // the final limb and store it as the last element of lower decomp so we can range check
    pub lower_decomp: Vec<T>,
}

pub struct IsLessThanCols<T> {
    pub io: IsLessThanIOCols<T>,
    pub aux: IsLessThanAuxCols<T>,
}

impl<T: Clone> IsLessThanCols<T> {
    pub fn from_slice(slc: &[T], limb_bits: usize, decomp: usize) -> Self {
        // num_limbs is the number of limbs, not including the last shifted limb
        let num_limbs = (limb_bits + decomp - 1) / decomp;

        // the first and second elements are x and y, respectively
        let x = slc[0].clone();
        let y = slc[1].clone();
        // the third element is the less_than indicator
        let less_than = slc[2].clone();

        // the next element is the value of the lower num_limbs bits of the intermediate sum
        let lower = slc[3].clone();

        // the next num_limbs + 1 elements are the decomposed limbs of the lower bits of the
        // intermediate sum
        let lower_decomp = slc[4..4 + num_limbs + 1].to_vec();

        let io = IsLessThanIOCols { x, y, less_than };
        let aux = IsLessThanAuxCols {
            lower,
            lower_decomp,
        };

        Self { io, aux }
    }

    pub fn flatten(&self) -> Vec<T> {
        let mut flattened = vec![
            self.io.x.clone(),
            self.io.y.clone(),
            self.io.less_than.clone(),
            self.aux.lower.clone(),
        ];
        flattened.extend(self.aux.lower_decomp.iter().cloned());
        flattened
    }

    pub fn get_width(limb_bits: usize, decomp: usize) -> usize {
        let mut width = 0;
        // for the x and y
        width += 2;
        // for the less_than indicator
        width += 1;
        // for the lower
        width += 1;
        // for the decomposed lower
        let num_limbs = (limb_bits + decomp - 1) / decomp;
        width += num_limbs + 1;

        width
    }
}
