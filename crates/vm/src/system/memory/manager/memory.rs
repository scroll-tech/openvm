use std::{
    array,
    cmp::max,
    collections::{
        BTreeMap,
        Bound::{Excluded, Included, Unbounded},
    },
    fmt::Debug,
};

use ax_stark_backend::p3_field::PrimeField32;
use rustc_hash::FxHashMap;

use crate::system::memory::{Equipartition, TimestampedEquipartition, TimestampedValues};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessAdapterRecordKind {
    Split,
    Merge {
        left_timestamp: u32,
        right_timestamp: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessAdapterRecord<T> {
    pub timestamp: u32,
    pub address_space: T,
    pub start_index: T,
    pub data: Vec<T>,
    pub kind: AccessAdapterRecordKind,
}

/// Represents a single or batch memory write operation.
/// Can be used to generate [MemoryWriteAuxCols].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemoryWriteRecord<T, const N: usize> {
    pub address_space: T,
    pub pointer: T,
    pub timestamp: u32,
    pub prev_timestamp: u32,
    pub data: [T; N],
    pub prev_data: [T; N],
}

impl<T: Copy> MemoryWriteRecord<T, 1> {
    pub fn value(&self) -> T {
        self.data[0]
    }
}

/// Represents a single or batch memory read operation.
///
/// Also used for "reads" from address space 0 (immediates).
/// Can be used to generate [MemoryReadAuxCols] or [MemoryReadOrImmediateAuxCols].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemoryReadRecord<T, const N: usize> {
    pub address_space: T,
    pub pointer: T,
    pub timestamp: u32,
    pub prev_timestamp: u32,
    pub data: [T; N],
}

impl<T: Copy> MemoryReadRecord<T, 1> {
    pub fn value(&self) -> T {
        self.data[0]
    }
}

pub const INITIAL_TIMESTAMP: u32 = 0;

/// (address_space, pointer)
type Address = (usize, usize);

#[derive(Debug)]
pub struct Memory<F> {
    ts_map: BTreeMap<Address, (usize, u32)>,
    data: FxHashMap<Address, F>,
    initial_block_size: usize,
    timestamp: u32,
}

impl<F: PrimeField32> Memory<F> {
    /// Creates a new partition with the given initial block size.
    ///
    /// Panics if the initial block size is not a power of two.
    pub fn new<const N: usize>(initial_memory: &Equipartition<F, N>) -> Self {
        assert!(N.is_power_of_two());

        let mut ts_map = BTreeMap::new();
        let mut data = FxHashMap::default();
        for (&(address_space, block_idx), values) in initial_memory {
            let address_space_usize = address_space.as_canonical_u32() as usize;
            let pointer = block_idx * N;
            ts_map.insert((address_space_usize, pointer), (N, INITIAL_TIMESTAMP));
            for (i, value) in values.iter().enumerate() {
                data.insert((address_space_usize, pointer + i), *value);
            }
        }
        Self {
            ts_map,
            data,
            initial_block_size: N,
            timestamp: INITIAL_TIMESTAMP + 1,
        }
    }

    pub fn timestamp(&self) -> u32 {
        self.timestamp
    }

    /// Increments the current timestamp by one and returns the new value.
    pub fn increment_timestamp(&mut self) {
        self.timestamp += 1;
    }

    /// Increments the current timestamp by a specified delta and returns the new value.
    pub fn increment_timestamp_by(&mut self, delta: u32) {
        self.timestamp += delta;
    }

    /// Writes an array of values to the memory at the specified address space and start index.
    pub fn write<const N: usize>(
        &mut self,
        address_space: usize,
        pointer: usize,
        values: [F; N],
    ) -> (MemoryWriteRecord<F, N>, Vec<AccessAdapterRecord<F>>) {
        assert!(N.is_power_of_two());

        let mut adapter_records = vec![];
        let prev_timestamp = self.access(
            address_space,
            pointer,
            N,
            &mut adapter_records,
            Some(self.timestamp),
        );
        debug_assert!(prev_timestamp < self.timestamp);

        let prev_data = array::from_fn(|i| {
            self.data
                .insert((address_space, pointer + i), values[i])
                .unwrap_or(F::ZERO)
        });

        let record = MemoryWriteRecord {
            address_space: F::from_canonical_usize(address_space),
            pointer: F::from_canonical_usize(pointer),
            timestamp: self.timestamp,
            prev_timestamp,
            data: values,
            prev_data,
        };
        self.increment_timestamp();
        (record, adapter_records)
    }

    /// Reads an array of values from the memory at the specified address space and start index.
    pub fn read<const N: usize>(
        &mut self,
        address_space: usize,
        pointer: usize,
    ) -> (MemoryReadRecord<F, N>, Vec<AccessAdapterRecord<F>>) {
        assert!(N.is_power_of_two());

        let mut adapter_records = vec![];
        let prev_timestamp = self.access(
            address_space,
            pointer,
            N,
            &mut adapter_records,
            Some(self.timestamp),
        );
        debug_assert!(prev_timestamp < self.timestamp);

        let record = MemoryReadRecord {
            address_space: F::from_canonical_usize(address_space),
            pointer: F::from_canonical_usize(pointer),
            timestamp: self.timestamp,
            prev_timestamp,
            data: self.range_array::<N>(address_space, pointer),
        };

        self.increment_timestamp();
        (record, adapter_records)
    }

    pub fn finalize<const N: usize>(
        &mut self,
    ) -> (TimestampedEquipartition<F, N>, Vec<AccessAdapterRecord<F>>) {
        let mut adapter_records = vec![];

        // First make sure the partition we maintain in self.blocks is an equipartition.
        let mut left = Unbounded;
        let mut range = self.ts_map.range((left, Unbounded));
        while let Some((&(address_space, pointer), &(size, _))) = range.next() {
            let aligned_pointer = (pointer / N) * N;
            //println!(
            //    "FINALIZE left: {:?} aligned_pointer: {:?} address_space: {:?} pointer: {:?} size: {:?} ts: {:?}",
            //    left, aligned_pointer, address_space, pointer, size, ts
            //);
            if aligned_pointer != pointer || size != N {
                self.access(
                    address_space,
                    aligned_pointer,
                    N,
                    &mut adapter_records,
                    None,
                );
            }
            left = Included((address_space, aligned_pointer + N));
            range = self.ts_map.range((left, Unbounded));
        }

        let mut equipartition = TimestampedEquipartition::<F, N>::new();
        for (&(address_space, pointer), &(_, ts)) in self.ts_map.iter() {
            debug_assert_eq!(pointer % N, 0);

            equipartition.insert(
                (F::from_canonical_usize(address_space), pointer / N),
                TimestampedValues {
                    timestamp: ts,
                    values: self.range_array::<N>(address_space, pointer),
                },
            );
        }

        //println!("equipartition: {:?}", equipartition);
        //println!("adapter_records: {:?}", adapter_records);
        (equipartition, adapter_records)
    }

    /// Returns the previous timestamp of the block starting at `pointer`
    fn access(
        &mut self,
        address_space: usize,
        pointer: usize,
        size: usize,
        records: &mut Vec<AccessAdapterRecord<F>>,
        new_ts: Option<u32>,
    ) -> u32 {
        //println!(
        //    "ACCESS address_space: {:?} pointer: {:?} size: {:?}",
        //    address_space, pointer, size
        //);
        let mut raw_rem_ptrs: Vec<(usize, usize, u32)> = vec![];

        // contains all pointers in the range (pointer, pointer + size)
        let range = self
            .ts_map
            .range((
                Excluded((address_space, pointer)),
                Excluded((address_space, pointer + size)),
            ))
            .peekable();

        for (val, &(size, ts)) in range {
            raw_rem_ptrs.push((val.1, size, ts));
        }
        //println!("raw_rem_ptrs: {:?}", raw_rem_ptrs);

        // contains the start of the largest block starting <= pointer
        let mut lower_range = self
            .ts_map
            .range((Unbounded, Included((address_space, pointer))));
        let (prev_ptr, prev_ts, prev_size, is_lower_initialized) =
            lower_range.next_back().map_or_else(
                || {
                    (
                        (pointer / self.initial_block_size) * self.initial_block_size,
                        INITIAL_TIMESTAMP,
                        self.initial_block_size,
                        false,
                    )
                },
                |(key, &(size, ts))| {
                    //println!("KEY LOW {:?}", key);
                    if key.0 != address_space
                        || key.1 + size
                            <= (pointer / self.initial_block_size) * self.initial_block_size
                    {
                        (
                            (pointer / self.initial_block_size) * self.initial_block_size,
                            INITIAL_TIMESTAMP,
                            self.initial_block_size,
                            false,
                        )
                    } else {
                        (key.1, ts, size, true)
                    }
                },
            );

        // contains the start of the smallest block starting >= pointer + size
        let mut upper_range = self
            .ts_map
            .range((Included((address_space, pointer + size)), Unbounded));
        let next_start = ((pointer + size + self.initial_block_size - 1) / self.initial_block_size)
            * self.initial_block_size;
        let initial_start = if let Some((last_ptr, last_size, ts)) = raw_rem_ptrs.last() {
            //println!(
            //    "AAAAA last_ptr: {:?} last_size: {:?} next_start: {:?}",
            //    last_ptr, last_size, next_start
            //);
            max(last_ptr + last_size, next_start)
        } else {
            //println!(
            //    "AAAAA prev_ptr: {:?} prev_size: {:?} next_start: {:?}",
            //    prev_ptr, prev_size, next_start
            //);
            max(prev_ptr + prev_size, next_start)
        };
        let (next_ptr, _, next_size, is_upper_initialized) = upper_range.next().map_or_else(
            || {
                (
                    initial_start,
                    INITIAL_TIMESTAMP,
                    self.initial_block_size,
                    false,
                )
            },
            |(key, &(new_size, new_ts))| {
                //println!(
                //    "BBBBB key: {:?} new_size: {:?} new_ts: {:?}",
                //    key, new_size, new_ts
                // );
                if key.0 != address_space || key.1 > initial_start {
                    (
                        initial_start,
                        INITIAL_TIMESTAMP,
                        self.initial_block_size,
                        true,
                    )
                } else {
                    (key.1, new_ts, new_size, true)
                }
            },
        );

        if is_lower_initialized {
            self.ts_map.remove(&(address_space, prev_ptr));
        }
        for (ptr, _, _) in &raw_rem_ptrs {
            self.ts_map.remove(&(address_space, *ptr));
        }

        // at this stage we know all blocks overlapping the range are:
        //   * [prev_ptr, prev_ptr + prev_size)
        //   * blocks in raw_rem_ptrs
        //   * initialized blocks skipped by raw_rem_ptrs
        // in addition, we know the next block is:
        //   * [next_ptr, next_ptr + next_size)
        //
        // we now iterate over the blocks in raw_rem_ptrs and explicitly list all the blocks
        // that are not initialized.
        let mut rem_ptrs = vec![];
        if prev_ptr == pointer {
            rem_ptrs.push((prev_ptr, prev_size, prev_ts));
        }
        let mut running_ptr = prev_ptr + prev_size;
        //println!(
        //    "ACCESS running_ptr: {:?} raw_rem_ptrs: {:?} next_ptr: {:?}",
        //    running_ptr, raw_rem_ptrs, next_ptr
        //);
        for (ptr, size, ts) in raw_rem_ptrs {
            if running_ptr < ptr {
                assert!(
                    (ptr - running_ptr) % self.initial_block_size == 0,
                    "ptr: {:?} running_ptr: {:?} initial_block_size: {:?}",
                    ptr,
                    running_ptr,
                    self.initial_block_size
                );
                for i in 0..(ptr - running_ptr) / self.initial_block_size {
                    rem_ptrs.push((
                        running_ptr + i * self.initial_block_size,
                        self.initial_block_size,
                        INITIAL_TIMESTAMP,
                    ));
                }
            }
            rem_ptrs.push((ptr, size, ts));
            running_ptr = ptr + size;
        }

        if running_ptr < next_ptr {
            assert!(
                (next_ptr - running_ptr) % self.initial_block_size == 0,
                "next_ptr: {:?} running_ptr: {:?} initial_block_size: {:?}",
                next_ptr,
                running_ptr,
                self.initial_block_size
            );
            for i in 0..(next_ptr - running_ptr) / self.initial_block_size {
                rem_ptrs.push((
                    running_ptr + i * self.initial_block_size,
                    self.initial_block_size,
                    INITIAL_TIMESTAMP,
                ));
            }
        }

        //println!(
        //    "ACCESS prev_ptr: {:?} prev_size: {:?} next_ptr: {:?} next_size: {:?}",
        //     prev_ptr, prev_size, next_ptr, next_size
        // );
        //println!("pointer: {:?} size: {:?}", pointer, size);
        //println!("rem_ptrs: {:?}", rem_ptrs);
        let ret_ts = if rem_ptrs.len() == 0 || (rem_ptrs.len() == 1 && prev_ptr == pointer) {
            // we need to split up a single block at the boundaries:
            // [prev_ptr, next_ptr) at [pointer, pointer + size)
            // we know that
            //     prev_ptr <= pointer < pointer + size <= next_ptr

            self.split_twice(
                address_space,
                prev_ptr,
                prev_ts,
                next_ptr,
                pointer,
                pointer + size,
                records,
            );

            self.ts_map
                .insert((address_space, pointer), (size, new_ts.unwrap_or(prev_ts)));
            prev_ts
        } else {
            // we need to split up the blocks at the boundaries:
            // [prev_ptr, rem_ptrs[0]) at pointer
            // [rem_ptrs[-1], next_ptr) at pointer + size

            let mut left_split = if prev_ptr < rem_ptrs[0].0 {
                self.split(
                    address_space,
                    prev_ptr,
                    prev_ts,
                    rem_ptrs[0].0,
                    pointer,
                    records,
                    true,
                    false,
                )
            } else {
                vec![]
            };
            //println!(
            //    "right_split rem_ptrs: {:?} next_ptr: {:?} pointer + size: {:?}",
            //    rem_ptrs,
            //    next_ptr,
            //    pointer + size
            //);
            let right_split = self.split(
                address_space,
                rem_ptrs[rem_ptrs.len() - 1].0,
                rem_ptrs[rem_ptrs.len() - 1].2,
                next_ptr,
                pointer + size,
                records,
                false,
                true,
            );

            // we need to merge the blocks on the interior:
            // [pointer, rem_ptrs[0], ..., rem_ptrs[-1], pointer + size)
            //println!(
            //    "merge left_split: {:?} rem_ptrs: {:?} right_split: {:?}",
            //    left_split, rem_ptrs, right_split
            //);
            left_split.extend(rem_ptrs[..rem_ptrs.len() - 1].to_vec());
            if rem_ptrs[rem_ptrs.len() - 1].0 + rem_ptrs[rem_ptrs.len() - 1].1 <= pointer + size {
                left_split.push(rem_ptrs[rem_ptrs.len() - 1]);
            }
            left_split.extend(right_split);
            //println!(
            //    "merge left_split: {:?} pointer: {:?} end_ptr: {:?}",
            //    left_split,
            //    pointer,
            //    pointer + size
            //);
            let merged_ts = self.merge(address_space, left_split, pointer + size, records);

            self.ts_map.insert(
                (address_space, pointer),
                (size, new_ts.unwrap_or(merged_ts)),
            );
            merged_ts
        };
        ret_ts
    }

    // Returns the timestamp of the merged block.
    /// entries is a list of (pointer, timestamp) pairs, sorted by pointer.
    /// end_ptr must be greater than all pointers in entries.
    fn merge(
        &mut self,
        address_space: usize,
        entries: Vec<(usize, usize, u32)>,
        end_ptr: usize,
        records: &mut Vec<AccessAdapterRecord<F>>,
    ) -> u32 {
        //println!("merge entries: {:?} end_ptr: {:?}", entries, end_ptr);
        // we first split only so that all entries are aligned.
        let mut aligned_entries = Vec::with_capacity(2 * entries.len());
        let align_start = entries[0].0;
        for (ptr, size, ts) in entries {
            let (aligned_ptrs, aligned_size) =
                self.aligned_split(address_space, align_start, ptr, ts, size, records);
            aligned_entries.extend(aligned_ptrs.iter().map(|&ptr| (ptr, aligned_size, ts)));
        }

        // we then merge blocks
        let mut merged_entries: Vec<(usize, u32, usize)> =
            Vec::with_capacity(aligned_entries.len());
        //println!("MERGE aligned_entries: {:?}", aligned_entries);
        for (ptr, size, ts) in aligned_entries {
            //println!(
            //    "MERGE merged_entries: {:?} ptr: {:?} size: {:?} ts: {:?}",
            //    merged_entries, ptr, size, ts
            //);
            if merged_entries.len() == 0 || (*(merged_entries.last().unwrap())).2 > size {
                merged_entries.push((ptr, ts, size));
            } else {
                assert!(merged_entries.last().unwrap().2 == size);
                let mut size = size;
                let mut ts = ts;
                let mut final_ptr = ptr;
                while let Some((ptr, merged_ts, merged_size)) = merged_entries.pop() {
                    if merged_size == size {
                        let new_ts = max(merged_ts, ts);
                        records.push(AccessAdapterRecord {
                            timestamp: new_ts,
                            address_space: F::from_canonical_usize(address_space),
                            start_index: F::from_canonical_usize(ptr),
                            data: self.range_vec(address_space, ptr, 2 * size),
                            kind: AccessAdapterRecordKind::Merge {
                                left_timestamp: merged_ts,
                                right_timestamp: ts,
                            },
                        });
                        size = 2 * size;
                        final_ptr = ptr;
                        ts = new_ts;
                    } else {
                        merged_entries.push((ptr, merged_ts, merged_size));
                        assert!(merged_size > size);
                        break;
                    }
                }
                merged_entries.push((final_ptr, ts, size));
            }
        }
        merged_entries.last().unwrap().1
    }

    // Returns the new pointers for a split of a block of size `size` starting at `start_offset` which is
    // split to only have bit-aligned blocks.
    fn aligned_split(
        &mut self,
        address_space: usize,
        align_start: usize,
        start_offset: usize,
        start_ts: u32,
        size: usize,
        records: &mut Vec<AccessAdapterRecord<F>>,
    ) -> (Vec<usize>, usize) {
        //println!(
        //    "ALIGNED SPLIT address_space: {:?} align_start: {:?} start_offset: {:?} size: {:?}",
        //    address_space, align_start, start_offset, size
        //);
        assert!(size.is_power_of_two());
        let start_trailing_bits = (start_offset - align_start).trailing_zeros();
        let size_bits = size.trailing_zeros();

        if start_trailing_bits >= size_bits {
            // no splitting is needed
            return (vec![start_offset], size);
        } else {
            for split_bits in start_trailing_bits..size_bits {
                for idx in 0..(1 << (split_bits - start_trailing_bits)) {
                    records.push(AccessAdapterRecord {
                        timestamp: start_ts,
                        address_space: F::from_canonical_usize(address_space),
                        start_index: F::from_canonical_usize(
                            start_offset + idx * (1 << (split_bits + 1)),
                        ),
                        data: self.range_vec(
                            address_space,
                            start_offset + idx * (1 << (split_bits + 1)),
                            1 << (split_bits + 1),
                        ),
                        kind: AccessAdapterRecordKind::Split,
                    });
                }
            }
            return (
                (0..(1 << (size_bits - start_trailing_bits)))
                    .map(|i| start_offset + i * (1 << start_trailing_bits))
                    .collect(),
                1 << start_trailing_bits,
            );
        }
    }

    // Returns new entries which are not inserted into the ts_map, according to the add_left flag.
    fn split(
        &mut self,
        address_space: usize,
        start_ptr: usize,
        start_ts: u32,
        end_ptr: usize,
        split_ptr: usize,
        records: &mut Vec<AccessAdapterRecord<F>>,
        add_left: bool,
        add_right: bool,
    ) -> Vec<(usize, usize, u32)> {
        //println!(
        //    "SPLIT start_ptr: {:?} split_ptr: {:?} end_ptr: {:?}",
        //    start_ptr, split_ptr, end_ptr
        //);
        assert!(start_ptr <= split_ptr && split_ptr <= end_ptr);
        assert!((end_ptr - start_ptr).is_power_of_two());

        let mut start_ptr = start_ptr;
        let mut end_ptr = end_ptr;
        let mut new_entries = vec![];
        while start_ptr != split_ptr && end_ptr != split_ptr {
            records.push(AccessAdapterRecord {
                timestamp: start_ts,
                address_space: F::from_canonical_usize(address_space),
                start_index: F::from_canonical_usize(start_ptr),
                data: self.range_vec(address_space, start_ptr, end_ptr - start_ptr),
                kind: AccessAdapterRecordKind::Split,
            });
            let mid_ptr = (start_ptr + end_ptr) / 2;
            if mid_ptr < split_ptr {
                if add_left {
                    self.ts_map
                        .insert((address_space, start_ptr), (mid_ptr - start_ptr, start_ts));
                } else {
                    new_entries.push((start_ptr, mid_ptr - start_ptr, start_ts));
                }
                start_ptr = mid_ptr;
            } else {
                if add_right {
                    self.ts_map
                        .insert((address_space, mid_ptr), (end_ptr - mid_ptr, start_ts));
                } else {
                    new_entries.push((mid_ptr, end_ptr - mid_ptr, start_ts));
                }
                end_ptr = mid_ptr;
            }
        }
        new_entries.sort_by_key(|&(ptr, _, _)| ptr);
        new_entries
    }

    fn split_twice(
        &mut self,
        address_space: usize,
        start_ptr: usize,
        start_ts: u32,
        end_ptr: usize,
        split_ptr_left: usize,
        split_ptr_right: usize,
        records: &mut Vec<AccessAdapterRecord<F>>,
    ) {
        //println!(
        //    "SPLIT TWICE start_ptr: {:?} split_ptr_left: {:?} split_ptr_right: {:?} end_ptr: {:?}",
        //    start_ptr, split_ptr_left, split_ptr_right, end_ptr
        //);
        assert!(
            start_ptr <= split_ptr_left
                && split_ptr_right <= end_ptr
                && split_ptr_left < split_ptr_right
        );
        assert!(
            (end_ptr - start_ptr).is_power_of_two(),
            "end_ptr: {:?} start_ptr: {:?}",
            end_ptr,
            start_ptr
        );
        assert!(
            (split_ptr_right - split_ptr_left).is_power_of_two(),
            "split_ptr_right: {:?} split_ptr_left: {:?}",
            split_ptr_right,
            split_ptr_left
        );

        let mut start_ptr = start_ptr;
        let mut end_ptr = end_ptr;
        let mut mid_ptr_opt = None;
        while start_ptr != split_ptr_left || end_ptr != split_ptr_right {
            records.push(AccessAdapterRecord {
                timestamp: start_ts,
                address_space: F::from_canonical_usize(address_space),
                start_index: F::from_canonical_usize(start_ptr),
                data: self.range_vec(address_space, start_ptr, end_ptr - start_ptr),
                kind: AccessAdapterRecordKind::Split,
            });
            let mid_ptr = (start_ptr + end_ptr) / 2;
            if mid_ptr <= split_ptr_left {
                //println!(
                //    "INSERT as {:?} start_ptr: {:?} size: {:?}",
                //    address_space,
                //    start_ptr,
                //    mid_ptr - start_ptr
                //);
                self.ts_map
                    .insert((address_space, start_ptr), (mid_ptr - start_ptr, start_ts));
                start_ptr = mid_ptr;
            } else if mid_ptr >= split_ptr_right {
                //println!(
                //    "INSERT as {:?} start_ptr: {:?} size: {:?}",
                //    address_space,
                //    mid_ptr,
                //    end_ptr - mid_ptr
                //);
                self.ts_map
                    .insert((address_space, mid_ptr), (end_ptr - mid_ptr, start_ts));
                end_ptr = mid_ptr;
            } else {
                mid_ptr_opt = Some(mid_ptr);
                break;
            }
        }
        if mid_ptr_opt.is_none() {
            // this means that splitting leaves [split_ptr_left, split_ptr_right)
            // unchanged, so we can just return.
            return;
        }

        let mut left_split = self.split(
            address_space,
            start_ptr,
            start_ts,
            mid_ptr_opt.unwrap(),
            split_ptr_left,
            records,
            true,
            false,
        );
        let right_split = self.split(
            address_space,
            mid_ptr_opt.unwrap(),
            start_ts,
            end_ptr,
            split_ptr_right,
            records,
            false,
            true,
        );
        left_split.extend(right_split);

        // merge from [split_ptr_left, mid_ptr) and [mid_ptr, split_ptr_right)
        //println!(
        //    "SPLIT_TWICE left_split: {:?} split_ptr_right {:?}",
        //    left_split, split_ptr_right
        //);
        self.merge(address_space, left_split, split_ptr_right, records);
    }

    pub fn get(&self, address_space: usize, pointer: usize) -> F {
        *self.data.get(&(address_space, pointer)).unwrap_or(&F::ZERO)
    }

    fn range_array<const N: usize>(&self, address_space: usize, pointer: usize) -> [F; N] {
        array::from_fn(|i| self.get(address_space, pointer + i))
    }

    fn range_vec(&self, address_space: usize, pointer: usize, len: usize) -> Vec<F> {
        (0..len)
            .map(|i| self.get(address_space, pointer + i))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use ax_stark_backend::p3_field::AbstractField;
    use ax_stark_sdk::p3_baby_bear::BabyBear;

    use super::Memory;
    use crate::system::memory::{
        manager::memory::{AccessAdapterRecord, AccessAdapterRecordKind},
        Equipartition, MemoryReadRecord, MemoryWriteRecord, TimestampedValues,
    };

    macro_rules! bb {
        ($x:expr) => {
            BabyBear::from_canonical_u32($x)
        };
    }

    macro_rules! bba {
        [$($x:expr),*] => {
            [$(BabyBear::from_canonical_u32($x)),*]
        }
    }

    macro_rules! bbvec {
        [$($x:expr),*] => {
            vec![$(BabyBear::from_canonical_u32($x)),*]
        }
    }

    /*
    #[test]
    fn test_partition() {
        type F = BabyBear;

        let mut partition = Memory::<F>::new(&Equipartition::<F, 8>::new());
        assert_eq!(partition.block_containing(0, 13), (8, 8, 0));
        assert_eq!(partition.block_containing(0, 8), (8, 8, 0));
        assert_eq!(partition.block_containing(0, 15), (8, 8, 0));
        assert_eq!(partition.block_containing(0, 16), (16, 8, 0));
    }
    */

    #[test]
    fn test_write_read_initial_block_len_1() {
        let initial_memory = Equipartition::<BabyBear, 1>::new();
        let mut memory = Memory::<BabyBear>::new(&initial_memory);
        let address_space = 1;

        memory.write(address_space, 0, bba![1, 2, 3, 4]);

        let (read_record, _) = memory.read::<2>(address_space, 0);
        assert_eq!(read_record.data, bba![1, 2]);

        memory.write(address_space, 2, bba![100]);

        let (read_record, _) = memory.read::<4>(address_space, 0);
        assert_eq!(read_record.data, bba![1, 2, 100, 4]);
    }

    #[test]
    fn test_write_read_initial_block_len_8() {
        let initial_memory = Equipartition::<BabyBear, 8>::new();
        let mut memory = Memory::<BabyBear>::new(&initial_memory);
        let address_space = 1;

        memory.write(address_space, 0, bba![1, 2, 3, 4]);

        let (read_record, _) = memory.read::<2>(address_space, 0);
        assert_eq!(read_record.data, bba![1, 2]);

        memory.write(address_space, 2, bba![100]);

        let (read_record, _) = memory.read::<4>(address_space, 0);
        assert_eq!(read_record.data, bba![1, 2, 100, 4]);
    }

    #[test]
    fn test_records_initial_block_len_1() {
        let initial_memory = Equipartition::<BabyBear, 1>::new();
        let mut memory = Memory::<BabyBear>::new(&initial_memory);

        let (write_record, adapter_records) = memory.write(1, 0, bba![1, 2, 3, 4]);

        // Above write first causes merge of [0:1] and [1:2] into [0:2].
        assert_eq!(
            adapter_records[0],
            AccessAdapterRecord {
                timestamp: 0,
                address_space: bb!(1),
                start_index: bb!(0),
                data: bbvec![0, 0],
                kind: AccessAdapterRecordKind::Merge {
                    left_timestamp: 0,
                    right_timestamp: 0,
                },
            }
        );
        // then merge [2:3] and [3:4] into [2:4].
        assert_eq!(
            adapter_records[1],
            AccessAdapterRecord {
                timestamp: 0,
                address_space: bb!(1),
                start_index: bb!(2),
                data: bbvec![0, 0],
                kind: AccessAdapterRecordKind::Merge {
                    left_timestamp: 0,
                    right_timestamp: 0,
                },
            }
        );
        // then merge [0:2] and [2:4] into [0:4].
        assert_eq!(
            adapter_records[2],
            AccessAdapterRecord {
                timestamp: 0,
                address_space: bb!(1),
                start_index: bb!(0),
                data: bbvec![0, 0, 0, 0],
                kind: AccessAdapterRecordKind::Merge {
                    left_timestamp: 0,
                    right_timestamp: 0,
                },
            }
        );
        // At time 1 we write [0:4].
        assert_eq!(
            write_record,
            MemoryWriteRecord {
                address_space: bb!(1),
                pointer: bb!(0),
                timestamp: 1,
                prev_timestamp: 0,
                data: bba![1, 2, 3, 4],
                prev_data: bba![0, 0, 0, 0],
            }
        );
        assert_eq!(memory.timestamp(), 2);

        let (read_record, adapter_records) = memory.read::<4>(1, 0);
        // At time 2 we read [0:4].
        assert_eq!(adapter_records.len(), 0);
        assert_eq!(
            read_record,
            MemoryReadRecord {
                address_space: bb!(1),
                pointer: bb!(0),
                timestamp: 2,
                prev_timestamp: 1,
                data: bba![1, 2, 3, 4],
            }
        );
        assert_eq!(memory.timestamp(), 3);

        let (read_record, adapter_records) = memory.write::<2>(1, 0, bba![10, 11]);
        // write causes split [0:4] into [0:2] and [2:4] (to prepare for write to [0:2]).
        assert_eq!(adapter_records.len(), 1);
        assert_eq!(
            adapter_records[0],
            AccessAdapterRecord {
                timestamp: 2,
                address_space: bb!(1),
                start_index: bb!(0),
                data: bbvec![1, 2, 3, 4],
                kind: AccessAdapterRecordKind::Split,
            }
        );

        // At time 3 we write [10, 11] into [0, 2].
        assert_eq!(
            read_record,
            MemoryWriteRecord {
                address_space: bb!(1),
                pointer: bb!(0),
                timestamp: 3,
                prev_timestamp: 2,
                data: bba![10, 11],
                prev_data: bba![1, 2],
            }
        );

        let (read_record, adapter_records) = memory.read::<4>(1, 0);
        assert_eq!(adapter_records.len(), 1);
        assert_eq!(
            adapter_records[0],
            AccessAdapterRecord {
                timestamp: 3,
                address_space: bb!(1),
                start_index: bb!(0),
                data: bbvec![10, 11, 3, 4],
                kind: AccessAdapterRecordKind::Merge {
                    left_timestamp: 3,
                    right_timestamp: 2
                },
            }
        );
        // At time 9 we read [0:4].
        assert_eq!(
            read_record,
            MemoryReadRecord {
                address_space: bb!(1),
                pointer: bb!(0),
                timestamp: 4,
                prev_timestamp: 3,
                data: bba![10, 11, 3, 4],
            }
        );
    }

    #[test]
    fn test_records_initial_block_len_8() {
        let initial_memory = Equipartition::<BabyBear, 8>::new();
        let mut memory = Memory::<BabyBear>::new(&initial_memory);

        let (write_record, adapter_records) = memory.write(1, 0, bba![1, 2, 3, 4]);

        // Above write first causes split of [0:8] into [0:4] and [4:8].
        assert_eq!(adapter_records.len(), 1);
        assert_eq!(
            adapter_records[0],
            AccessAdapterRecord {
                timestamp: 0,
                address_space: bb!(1),
                start_index: bb!(0),
                data: bbvec![0, 0, 0, 0, 0, 0, 0, 0],
                kind: AccessAdapterRecordKind::Split,
            }
        );
        // At time 1 we write [0:4].
        assert_eq!(
            write_record,
            MemoryWriteRecord {
                address_space: bb!(1),
                pointer: bb!(0),
                timestamp: 1,
                prev_timestamp: 0,
                data: bba![1, 2, 3, 4],
                prev_data: bba![0, 0, 0, 0],
            }
        );
        assert_eq!(memory.timestamp(), 2);

        let (read_record, adapter_records) = memory.read::<4>(1, 0);
        // At time 2 we read [0:4].
        assert_eq!(adapter_records.len(), 0);
        assert_eq!(
            read_record,
            MemoryReadRecord {
                address_space: bb!(1),
                pointer: bb!(0),
                timestamp: 2,
                prev_timestamp: 1,
                data: bba![1, 2, 3, 4],
            }
        );
        assert_eq!(memory.timestamp(), 3);

        let (read_record, adapter_records) = memory.write::<2>(1, 0, bba![10, 11]);
        // write causes split [0:4] into [0:2] and [2:4] (to prepare for write to [0:2]).
        assert_eq!(adapter_records.len(), 1);
        assert_eq!(
            adapter_records[0],
            AccessAdapterRecord {
                timestamp: 2,
                address_space: bb!(1),
                start_index: bb!(0),
                data: bbvec![1, 2, 3, 4],
                kind: AccessAdapterRecordKind::Split,
            }
        );

        // At time 3 we write [10, 11] into [0, 2].
        assert_eq!(
            read_record,
            MemoryWriteRecord {
                address_space: bb!(1),
                pointer: bb!(0),
                timestamp: 3,
                prev_timestamp: 2,
                data: bba![10, 11],
                prev_data: bba![1, 2],
            }
        );

        let (read_record, adapter_records) = memory.read::<4>(1, 0);
        assert_eq!(adapter_records.len(), 1);
        assert_eq!(
            adapter_records[0],
            AccessAdapterRecord {
                timestamp: 3,
                address_space: bb!(1),
                start_index: bb!(0),
                data: bbvec![10, 11, 3, 4],
                kind: AccessAdapterRecordKind::Merge {
                    left_timestamp: 3,
                    right_timestamp: 2
                },
            }
        );
        // At time 9 we read [0:4].
        assert_eq!(
            read_record,
            MemoryReadRecord {
                address_space: bb!(1),
                pointer: bb!(0),
                timestamp: 4,
                prev_timestamp: 3,
                data: bba![10, 11, 3, 4],
            }
        );
    }

    #[test]
    fn test_get_initial_block_len_1() {
        let initial_memory = Equipartition::<BabyBear, 1>::new();
        let mut memory = Memory::<BabyBear>::new(&initial_memory);

        memory.write(1, 0, bba![4, 3, 2, 1]);

        assert_eq!(memory.get(1, 0), BabyBear::from_canonical_u32(4));
        assert_eq!(memory.get(1, 1), BabyBear::from_canonical_u32(3));
        assert_eq!(memory.get(1, 2), BabyBear::from_canonical_u32(2));
        assert_eq!(memory.get(1, 3), BabyBear::from_canonical_u32(1));
        assert_eq!(memory.get(1, 5), BabyBear::ZERO);

        assert_eq!(memory.get(0, 0), BabyBear::ZERO);
    }

    #[test]
    fn test_get_initial_block_len_8() {
        let initial_memory = Equipartition::<BabyBear, 8>::new();
        let mut memory = Memory::<BabyBear>::new(&initial_memory);

        memory.write(1, 0, bba![4, 3, 2, 1]);

        assert_eq!(memory.get(1, 0), BabyBear::from_canonical_u32(4));
        assert_eq!(memory.get(1, 1), BabyBear::from_canonical_u32(3));
        assert_eq!(memory.get(1, 2), BabyBear::from_canonical_u32(2));
        assert_eq!(memory.get(1, 3), BabyBear::from_canonical_u32(1));
        assert_eq!(memory.get(1, 5), BabyBear::ZERO);
        assert_eq!(memory.get(1, 9), BabyBear::ZERO);
        assert_eq!(memory.get(0, 0), BabyBear::ZERO);
    }

    #[test]
    fn test_finalize_empty() {
        let initial_memory = Equipartition::<BabyBear, 4>::new();
        let mut memory = Memory::<BabyBear>::new(&initial_memory);

        let (memory, records) = memory.finalize::<4>();
        assert_eq!(memory.len(), 0);
        assert_eq!(records.len(), 0);
    }

    #[test]
    fn test_finalize_block_len_8() {
        let initial_memory = Equipartition::<BabyBear, 8>::new();
        let mut memory = Memory::<BabyBear>::new(&initial_memory);
        // Make block 0:4 in address space 1 active.
        memory.write(1, 0, bba![1, 2, 3, 4]);

        // Make block 16:32 in address space 1 active.
        memory.write(1, 16, bba![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);

        // Make block 64:72 in address space 2 active.
        memory.write(2, 64, bba![8, 7, 6, 5, 4, 3, 2, 1]);

        // Finalize to a partition of size 8.
        let (final_memory, records) = memory.finalize::<8>();
        assert_eq!(final_memory.len(), 4);
        assert_eq!(
            final_memory.get(&(bb!(1), 0)),
            Some(&TimestampedValues {
                values: bba![1, 2, 3, 4, 0, 0, 0, 0],
                timestamp: 1,
            })
        );
        // start_index = 16 corresponds to label = 2
        assert_eq!(
            final_memory.get(&(bb!(1), 2)),
            Some(&TimestampedValues {
                values: bba![1, 1, 1, 1, 1, 1, 1, 1],
                timestamp: 2,
            })
        );
        // start_index = 24 corresponds to label = 3
        assert_eq!(
            final_memory.get(&(bb!(1), 3)),
            Some(&TimestampedValues {
                values: bba![1, 1, 1, 1, 1, 1, 1, 1],
                timestamp: 2,
            })
        );
        // start_index = 64 corresponds to label = 8
        assert_eq!(
            final_memory.get(&(bb!(2), 8)),
            Some(&TimestampedValues {
                values: bba![8, 7, 6, 5, 4, 3, 2, 1],
                timestamp: 3,
            })
        );

        // We need to do 1 + 1 + 0 = 2 adapters.
        assert_eq!(records.len(), 2);
    }

    #[test]
    fn test_write_read_initial_block_len_8_initial_memory() {
        type F = BabyBear;

        // Initialize initial memory with blocks at indices 0 and 2
        let mut initial_memory = Equipartition::<F, 8>::new();
        initial_memory.insert((F::ONE, 0), bba![1, 2, 3, 4, 5, 6, 7, 8]); // Block 0, pointers 0–8
        initial_memory.insert((F::ONE, 2), bba![1, 2, 3, 4, 5, 6, 7, 8]); // Block 2, pointers 16–24

        let mut memory = Memory::new(&initial_memory);

        // Verify initial state of block 0 (pointers 0–8)
        let (initial_read_record_0, _) = memory.read::<8>(1, 0);
        assert_eq!(initial_read_record_0.data, bba![1, 2, 3, 4, 5, 6, 7, 8]);

        // Verify initial state of block 2 (pointers 16–24)
        let (initial_read_record_2, _) = memory.read::<8>(1, 16);
        assert_eq!(initial_read_record_2.data, bba![1, 2, 3, 4, 5, 6, 7, 8]);

        // Test: Write a partial block to block 0 (pointer 0) and read back partially and fully
        memory.write(1, 0, bba![9, 9, 9, 9]);
        let (partial_read_record, _) = memory.read::<2>(1, 0);
        assert_eq!(partial_read_record.data, bba![9, 9]);

        let (full_read_record_0, _) = memory.read::<8>(1, 0);
        assert_eq!(full_read_record_0.data, bba![9, 9, 9, 9, 5, 6, 7, 8]);

        // Test: Write a single element to pointer 2 and verify read in different lengths
        memory.write(1, 2, bba![100]);
        let (read_record_4, _) = memory.read::<4>(1, 1);
        assert_eq!(read_record_4.data, bba![9, 100, 9, 5]);

        let (full_read_record_2, _) = memory.read::<8>(1, 2);
        assert_eq!(full_read_record_2.data, bba![100, 9, 5, 6, 7, 8, 0, 0]);

        // Test: Write and read at the last pointer in block 2 (pointer 23, part of key (1, 2))
        memory.write(1, 23, bba![77]);
        let (boundary_read_record, _) = memory.read::<2>(1, 23);
        assert_eq!(boundary_read_record.data, bba![77, 0]); // Last byte modified, ensuring boundary check

        // Test: Reading from an uninitialized block (should default to 0)
        let (default_read_record, _) = memory.read::<4>(1, 10);
        assert_eq!(default_read_record.data, bba![0, 0, 0, 0]);

        let (default_read_record, _) = memory.read::<4>(1, 100);
        assert_eq!(default_read_record.data, bba![0, 0, 0, 0]);

        // Test: Overwrite entire memory pointer 16–24 and verify
        memory.write(1, 16, bba![50, 50, 50, 50, 50, 50, 50, 50]);
        let (overwrite_read_record, _) = memory.read::<8>(1, 16);
        assert_eq!(
            overwrite_read_record.data,
            bba![50, 50, 50, 50, 50, 50, 50, 50]
        ); // Verify entire block overwrite
    }
}
