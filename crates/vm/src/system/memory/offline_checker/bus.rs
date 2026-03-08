use std::iter;

use openvm_stark_backend::{
    interaction::{BusIndex, InteractionBuilder, LookupBus, PermutationCheckBus},
    p3_field::FieldAlgebra,
};

use crate::system::memory::MemoryAddress;

/// Represents a memory bus identified by a unique bus index (`usize`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemoryBus {
    pub inner: PermutationCheckBus,
}

impl MemoryBus {
    pub const fn new(index: BusIndex) -> Self {
        Self {
            inner: PermutationCheckBus::new(index),
        }
    }
}

impl MemoryBus {
    #[inline(always)]
    pub fn index(&self) -> BusIndex {
        self.inner.index
    }

    /// Prepares a write operation through the memory bus.
    pub fn send<T: Clone>(
        &self,
        address: MemoryAddress<impl Into<T>, impl Into<T>>,
        data: Vec<impl Into<T>>,
        timestamp: impl Into<T>,
    ) -> MemoryBusInteraction<T> {
        self.push(true, address, data, timestamp)
    }

    /// Prepares a read operation through the memory bus.
    pub fn receive<T: Clone>(
        &self,
        address: MemoryAddress<impl Into<T>, impl Into<T>>,
        data: Vec<impl Into<T>>,
        timestamp: impl Into<T>,
    ) -> MemoryBusInteraction<T> {
        self.push(false, address, data, timestamp)
    }

    /// Prepares a memory operation (read or write) through the memory bus.
    fn push<T: Clone>(
        &self,
        is_send: bool,
        address: MemoryAddress<impl Into<T>, impl Into<T>>,
        data: Vec<impl Into<T>>,
        timestamp: impl Into<T>,
    ) -> MemoryBusInteraction<T> {
        MemoryBusInteraction {
            bus: self.inner,
            is_send,
            address: MemoryAddress::new(address.address_space.into(), address.pointer.into()),
            data: data.into_iter().map(|item| item.into()).collect(),
            timestamp: timestamp.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MemoryBusInteraction<T> {
    pub bus: PermutationCheckBus,
    pub is_send: bool,
    pub address: MemoryAddress<T, T>,
    pub data: Vec<T>,
    pub timestamp: T,
}

impl<T: FieldAlgebra> MemoryBusInteraction<T> {
    /// Finalizes and sends/receives the memory operation with the specified direction over the bus.
    ///
    /// A read corresponds to a receive, and a write corresponds to a send.
    ///
    /// The parameter `direction` can be -1, 0, or 1. A value of 1 means perform the same action
    /// (send/receive), a value of -1 reverses the action, and a value of 0 means disabled.
    ///
    /// Caller must constrain `direction` to be in {-1, 0, 1}.
    pub fn eval<AB>(self, builder: &mut AB, direction: impl Into<AB::Expr>)
    where
        AB: InteractionBuilder<Expr = T>,
    {
        let fields = iter::empty()
            .chain(iter::once(self.address.address_space))
            .chain(iter::once(self.address.pointer))
            .chain(self.data)
            .chain(iter::once(self.timestamp));

        if self.is_send {
            self.bus.interact(builder, fields, direction);
        } else {
            self.bus
                .interact(builder, fields, AB::Expr::NEG_ONE * direction.into());
        }
    }
}

/// Represents a hint bus identified by a unique bus index.
/// Used as a lookup table to constrain values read from hint space.
///
/// Consumer chips (e.g. NativeSumcheck) perform lookups to verify that
/// hint_space values match what was originally loaded.
/// Provider chips (e.g. a hint loader) add keys to the lookup table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HintBus {
    pub inner: LookupBus,
}

impl HintBus {
    pub const fn new(index: BusIndex) -> Self {
        Self {
            inner: LookupBus::new(index),
        }
    }

    #[inline(always)]
    pub fn index(&self) -> BusIndex {
        self.inner.index
    }

    /// Performs a lookup on the hint bus.
    ///
    /// Asserts that `(hint_id, offset, value)` is present in the hint lookup table.
    /// Caller must constrain that `enabled` is boolean.
    pub fn lookup<AB: InteractionBuilder>(
        &self,
        builder: &mut AB,
        hint_id: impl Into<AB::Expr>,
        offset: impl Into<AB::Expr>,
        value: impl Into<AB::Expr>,
        enabled: impl Into<AB::Expr>,
    ) {
        self.inner.lookup_key(
            builder,
            [hint_id.into(), offset.into(), value.into()],
            enabled,
        );
    }

    /// Adds a key to the hint lookup table.
    ///
    /// The `num_lookups` parameter should equal the number of enabled lookups performed
    /// for this key.
    pub fn provide<AB: InteractionBuilder>(
        &self,
        builder: &mut AB,
        hint_id: impl Into<AB::Expr>,
        offset: impl Into<AB::Expr>,
        value: impl Into<AB::Expr>,
        num_lookups: impl Into<AB::Expr>,
    ) {
        self.inner.add_key_with_lookups(
            builder,
            [hint_id.into(), offset.into(), value.into()],
            num_lookups,
        );
    }
}
