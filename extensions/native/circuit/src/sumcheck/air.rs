use openvm_circuit::{
    arch::{ExecutionBridge, ExecutionState},
    system::memory::{offline_checker::MemoryBridge, MemoryAddress, CHUNK},
};
use openvm_stark_backend::{
    air_builders::sub::SubAirBuilder,
    interaction::{BusIndex, InteractionBuilder, PermutationCheckBus},
    p3_air::{Air, AirBuilder, BaseAir},
    p3_field::{Field, FieldAlgebra},
    p3_matrix::Matrix,
    rap::{BaseAirWithPublicValues, PartitionedBaseAir},
};

#[derive(Clone, Debug)]
pub struct NativeSumcheckAir<F: Field> {
    pub execution_bridge: ExecutionBridge,
    pub memory_bridge: MemoryBridge,
    pub(crate) address_space: F,
}

impl<F: Field> BaseAir<F> for NativeSumcheckAir<F> {
    fn width(&self) -> usize {
        // _debug
        0
    }
}

impl<F: Field> BaseAirWithPublicValues<F>
    for NativeSumcheckAir<F>
{
}

impl<F: Field> PartitionedBaseAir<F>
    for NativeSumcheckAir<F>
{
}

impl<AB: InteractionBuilder> Air<AB>
    for NativeSumcheckAir<AB::F>
{
    fn eval(&self, builder: &mut AB) {
        // _debug
    }
}