use std::marker::PhantomData;

pub trait VanEmdeBoas<T: Clone>: Clone {
    type Iter<'a>: Iterator<Item = (u32, &'a T)>
    where
        T: 'a,
        Self: 'a;

    /// Constructs a new empty van Emde Boas tree.
    fn new() -> Self;

    /// Inserts a key-value pair into the tree.
    fn insert(&mut self, key: u32, value: T) -> Option<T>;

    /// Removes a key from the tree. The key must be present in the tree.
    fn erase(&mut self, key: u32);

    /// Returns the value associated with the given key.
    fn get(&self, key: u32) -> Option<&T>;

    /// Checks if a key is present in the tree.
    fn contains(&self, key: u32) -> bool;

    /// Returns an iterator over the tree.
    fn iter(&self) -> Self::Iter<'_>;

    /// Checks if the tree is empty.
    fn empty(&self) -> bool;

    /// Returns the smallest key in the tree.
    fn min(&self) -> Option<u32>;

    /// Returns the largest key in the tree.
    fn max(&self) -> Option<u32>;

    /// Returns the largest key in the tree less than or equal to the given key.
    fn max_not_exceeding(&self, key: u32) -> Option<u32>;
}

#[derive(Clone)]
pub struct VebLeaf<T: Clone> {
    values: [Option<T>; LEAF_SIZE],
}

impl<T: Clone> VanEmdeBoas<T> for VebLeaf<T> {
    type Iter<'a>
        = VebLeafIter<'a, T>
    where
        T: 'a,
        Self: 'a;

    fn new() -> Self {
        Self {
            values: [const { None }; LEAF_SIZE],
        }
    }

    fn insert(&mut self, key: u32, value: T) -> Option<T> {
        let key = key as usize;
        self.values[key].replace(value)
    }

    fn erase(&mut self, key: u32) {
        let key = key as usize;
        self.values[key] = None;
    }

    fn get(&self, key: u32) -> Option<&T> {
        let key = key as usize;
        self.values[key].as_ref()
    }

    fn contains(&self, key: u32) -> bool {
        let key = key as usize;
        self.values[key].is_some()
    }

    fn empty(&self) -> bool {
        self.values.iter().all(|v| v.is_none())
    }

    fn min(&self) -> Option<u32> {
        self.values
            .iter()
            .enumerate()
            .find_map(move |(i, v)| v.as_ref().map(|_| i as u32))
    }

    fn max(&self) -> Option<u32> {
        self.values
            .iter()
            .enumerate()
            .rev()
            .find_map(move |(i, v)| v.as_ref().map(|_| i as u32))
    }

    fn max_not_exceeding(&self, key: u32) -> Option<u32> {
        let mut key = key as usize;
        while key > 0 && self.values[key].is_none() {
            key -= 1;
        }
        if key > 0 {
            Some(key as u32)
        } else {
            None
        }
    }

    fn iter(&self) -> Self::Iter<'_> {
        VebLeafIter {
            values: &self.values,
            idx: 0,
        }
    }
}

pub struct VebLeafIter<'a, T: Clone> {
    values: &'a [Option<T>],
    idx: usize,
}

impl<'a, T: Clone> Iterator for VebLeafIter<'a, T> {
    type Item = (u32, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < LEAF_SIZE && self.values[self.idx].is_none() {
            self.idx += 1;
        }
        if self.idx < LEAF_SIZE {
            let item = self.values[self.idx].as_ref().unwrap();
            self.idx += 1;
            Some((self.idx as u32 - 1, item))
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct VebNode<const LOW_BITS: usize, const HIGH_CNT: usize, SmallKeys, HighKeys, T>
where
    SmallKeys: VanEmdeBoas<T>,
    HighKeys: VanEmdeBoas<()>,
    T: Clone,
{
    existing_children: HighKeys,
    subtrees: Vec<Option<SmallKeys>>,
    min: Option<u32>,
    max: Option<u32>,
    _phantom: PhantomData<T>,
}

impl<const LOW_BITS: usize, const HIGH_CNT: usize, SmallKeys, HighKeys, T>
    VebNode<LOW_BITS, HIGH_CNT, SmallKeys, HighKeys, T>
where
    SmallKeys: VanEmdeBoas<T>,
    HighKeys: VanEmdeBoas<()>,
    T: Clone,
{
    fn high(key: u32) -> u32 {
        key >> LOW_BITS
    }

    fn low(key: u32) -> u32 {
        key & ((1 << LOW_BITS) - 1)
    }
}

impl<const LOW_BITS: usize, const HIGH_CNT: usize, SmallKeys, HighKeys, T> VanEmdeBoas<T>
    for VebNode<LOW_BITS, HIGH_CNT, SmallKeys, HighKeys, T>
where
    SmallKeys: VanEmdeBoas<T>,
    HighKeys: VanEmdeBoas<()>,
    T: Clone,
{
    type Iter<'a>
        = VebNodeIter<'a, LOW_BITS, HIGH_CNT, SmallKeys, HighKeys, T>
    where
        T: 'a,
        Self: 'a;

    fn new() -> Self {
        Self {
            existing_children: HighKeys::new(),
            subtrees: vec![None; HIGH_CNT],
            min: None,
            max: None,
            _phantom: PhantomData,
        }
    }

    fn insert(&mut self, key: u32, value: T) -> Option<T> {
        self.existing_children.insert(Self::high(key), ());
        let result = self.subtrees[Self::high(key) as usize]
            .get_or_insert(SmallKeys::new())
            .insert(Self::low(key), value);
        if let Some(k) = self.min {
            if key < k {
                self.min = Some(key);
            }
        } else {
            self.min = Some(key);
        }
        if let Some(k) = self.max {
            if key > k {
                self.max = Some(key);
            }
        } else {
            self.max = Some(key);
        }
        result
    }

    fn erase(&mut self, key: u32) {
        self.subtrees[Self::high(key) as usize]
            .get_or_insert(SmallKeys::new())
            .erase(Self::low(key));
        if self.subtrees[Self::high(key) as usize]
            .as_ref()
            .unwrap()
            .empty()
        {
            self.existing_children.erase(Self::high(key));
        }
        if let Some(min_key) = self.min {
            if key == min_key {
                if let Some(max_key) = self.max {
                    if key == max_key {
                        self.min = None;
                        self.max = None;
                    } else {
                        let high =
                            <HighKeys as VanEmdeBoas<()>>::min(&self.existing_children).unwrap();
                        let low = self.subtrees[high as usize]
                            .as_ref()
                            .unwrap()
                            .min()
                            .unwrap();
                        self.min = Some(high << LOW_BITS | low);
                    }
                }
            }
        } else if let Some(max_key) = self.max {
            if key == max_key {
                let high = <HighKeys as VanEmdeBoas<()>>::max(&self.existing_children).unwrap();
                let low = self.subtrees[high as usize]
                    .as_ref()
                    .unwrap()
                    .max()
                    .unwrap();
                self.max = Some(high << LOW_BITS | low);
            }
        }
    }

    fn get(&self, key: u32) -> Option<&T> {
        self.subtrees[Self::high(key) as usize]
            .as_ref()?
            .get(Self::low(key))
    }

    fn contains(&self, key: u32) -> bool {
        self.subtrees[Self::high(key) as usize]
            .as_ref()
            .unwrap()
            .contains(Self::low(key))
    }

    fn iter(&self) -> Self::Iter<'_> {
        VebNodeIter {
            high_iter: self.existing_children.iter(),
            current_high: None,
            current_low_iter: None,
            subtrees: &self.subtrees,
        }
    }

    fn empty(&self) -> bool {
        self.min.is_none()
    }

    fn min(&self) -> Option<u32> {
        self.min
    }

    fn max(&self) -> Option<u32> {
        self.max
    }

    fn max_not_exceeding(&self, key: u32) -> Option<u32> {
        if let Some(subtree) = self.subtrees[Self::high(key) as usize].as_ref() {
            if let Some(low) = subtree.max_not_exceeding(Self::low(key)) {
                return Some(Self::high(key) << LOW_BITS | low);
            }
        }
        if let Some(high) = self.existing_children.max_not_exceeding(Self::high(key)) {
            let low = self.subtrees[high as usize]
                .as_ref()
                .unwrap()
                .max()
                .unwrap();
            Some(high << LOW_BITS | low)
        } else {
            None
        }
    }
}

pub struct VebNodeIter<'a, const LOW_BITS: usize, const HIGH_CNT: usize, SmallKeys, HighKeys, T>
where
    SmallKeys: VanEmdeBoas<T> + 'a,
    HighKeys: VanEmdeBoas<()> + 'a,
    T: Clone + 'a,
{
    high_iter: <HighKeys as VanEmdeBoas<()>>::Iter<'a>,
    current_high: Option<u32>,
    current_low_iter: Option<<SmallKeys as VanEmdeBoas<T>>::Iter<'a>>,
    subtrees: &'a [Option<SmallKeys>],
}

impl<'a, const LOW_BITS: usize, const HIGH_CNT: usize, SmallKeys, HighKeys, T> Iterator
    for VebNodeIter<'a, LOW_BITS, HIGH_CNT, SmallKeys, HighKeys, T>
where
    SmallKeys: VanEmdeBoas<T> + 'a,
    HighKeys: VanEmdeBoas<()> + 'a,
    T: Clone + 'a,
{
    type Item = (u32, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(low_iter) = &mut self.current_low_iter {
                if let Some((low, value)) = low_iter.next() {
                    return Some((self.current_high.unwrap() << LOW_BITS | low, value));
                }
                self.current_low_iter = None;
            }

            if let Some(high) = self.high_iter.next() {
                self.current_high = Some(high.0);
                self.current_low_iter =
                    Some(self.subtrees[high.0 as usize].as_ref().unwrap().iter());
                continue;
            }

            return None;
        }
    }
}

const LEAF_SIZE: usize = 1 << 8;
pub type _VebTree1<T> = VebNode<8, 256, VebLeaf<T>, VebLeaf<()>, T>;
pub type _VebTree2<T> = VebNode<16, 65536, _VebTree1<T>, _VebTree1<()>, T>;

pub type VebTree<T> = _VebTree2<T>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_veb_tree() {
        let mut tree = VebTree::<u32>::new();
        for i in 0..100 {
            tree.insert(i * i, i);
        }
        for i in 0..100 {
            assert_eq!(tree.get(i * i), Some(&i));
        }

        let v = tree.iter().collect::<Vec<_>>();
        assert_eq!(v.len(), 100);
        for (i, (key, value)) in v.into_iter().enumerate() {
            assert_eq!(key, (i * i) as u32);
            assert_eq!(*value, i as u32);
        }

        for i in 0..100 {
            tree.erase(i * i);
        }
        for i in 0..100 {
            assert_eq!(tree.get(i * i), None);
        }
    }
}
