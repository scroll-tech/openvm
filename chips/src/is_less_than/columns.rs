use afs_derive::AlignedBorrow;

#[derive(Default, AlignedBorrow)]
pub struct IsLessThanIOCols<T> {
    pub x: T,
    pub y: T,
    pub less_than: T,
}

pub struct IsLessThanAuxCols<T> {
    pub lower_bits: T,
    pub upper_bit: T,
    pub lower_bits_decomp: Vec<T>,
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
        let lower_bits = slc[3].clone();

        // the next element is the value of the upper bit of the intermediate sum; note that
        // y > x <=> upper_bit = 1
        let upper_bit = slc[4].clone();

        // the next num_limbs + 1 elements are the decomposed limbs of the lower bits of the
        // intermediate sum
        let lower_bits_decomp = slc[5..5 + num_limbs + 1].to_vec();

        let io = IsLessThanIOCols { x, y, less_than };
        let aux = IsLessThanAuxCols {
            lower_bits,
            upper_bit,
            lower_bits_decomp,
        };

        Self { io, aux }
    }

    pub fn flatten(&self) -> Vec<T> {
        let mut flattened = vec![
            self.io.x.clone(),
            self.io.y.clone(),
            self.io.less_than.clone(),
            self.aux.lower_bits.clone(),
            self.aux.upper_bit.clone(),
        ];
        flattened.extend(self.aux.lower_bits_decomp.iter().cloned());
        flattened
    }

    pub fn get_width(limb_bits: usize, decomp: usize) -> usize {
        let mut width = 0;
        // for the x and y
        width += 2;
        // for the less_than indicator
        width += 1;
        // for the lower_bits
        width += 1;
        // for the upper_bit
        width += 1;
        // for the decomposed lower_bits
        let num_limbs = (limb_bits + decomp - 1) / decomp;
        width += num_limbs + 1;

        width
    }
}
