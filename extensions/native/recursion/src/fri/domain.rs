use openvm_native_compiler::prelude::*;
use openvm_stark_backend::{
    p3_commit::{LagrangeSelectors, TwoAdicMultiplicativeCoset},
    p3_field::{Field, FieldAlgebra, TwoAdicField},
};

use super::types::FriConfigVariable;
use crate::commit::PolynomialSpaceVariable;

/// Reference: [`openvm_stark_backend::p3_commit::TwoAdicMultiplicativeCoset`]
#[derive(DslVariable, Clone)]
pub struct TwoAdicMultiplicativeCosetVariable<C: Config> {
    pub log_n: Usize<C::N>,
    pub size: Usize<C::N>,
    pub shift: Felt<C::F>,
    pub g: Felt<C::F>,
}

impl<C: Config> TwoAdicMultiplicativeCosetVariable<C> {
    pub fn size(&self) -> RVar<C::N> {
        self.size.clone().into()
    }

    pub fn first_point(&self) -> Felt<C::F> {
        self.shift
    }

    pub fn gen(&self) -> Felt<C::F> {
        self.g
    }
}

impl<C: Config> FromConstant<C> for TwoAdicMultiplicativeCosetVariable<C>
where
    C::F: TwoAdicField,
{
    type Constant = TwoAdicMultiplicativeCoset<C::F>;

    fn constant(value: Self::Constant, builder: &mut Builder<C>) -> Self {
        let g_val = C::F::two_adic_generator(value.log_n);
        TwoAdicMultiplicativeCosetVariable::<C> {
            // builder.eval is necessary to assign a variable in the dynamic mode.
            log_n: builder.eval(RVar::from(value.log_n)),
            size: builder.eval(RVar::from(1 << value.log_n)),
            shift: builder.eval(value.shift),
            g: builder.eval(g_val),
        }
    }
}

impl<C: Config> PolynomialSpaceVariable<C> for TwoAdicMultiplicativeCosetVariable<C>
where
    C::F: TwoAdicField,
{
    type Constant = TwoAdicMultiplicativeCoset<C::F>;

    fn next_point(
        &self,
        builder: &mut Builder<C>,
        point: Ext<<C as Config>::F, <C as Config>::EF>,
    ) -> Ext<<C as Config>::F, <C as Config>::EF> {
        builder.eval(point * self.gen())
    }

    fn selectors_at_point(
        &self,
        builder: &mut Builder<C>,
        point: Ext<<C as Config>::F, <C as Config>::EF>,
    ) -> LagrangeSelectors<Ext<<C as Config>::F, <C as Config>::EF>> {
        let unshifted_point: Ext<_, _> = builder.eval(point * self.shift.inverse());
        let z_h_expr =
            builder.exp_power_of_2_v::<Ext<_, _>>(unshifted_point, self.log_n.clone()) - C::EF::ONE;
        let z_h: Ext<_, _> = builder.eval(z_h_expr);

        LagrangeSelectors {
            is_first_row: builder.eval(z_h / (unshifted_point - C::EF::ONE)),
            is_last_row: builder.eval(z_h / (unshifted_point - self.gen().inverse())),
            is_transition: builder.eval(unshifted_point - self.gen().inverse()),
            inv_zeroifier: builder.eval(z_h.inverse()),
        }
    }

    fn zp_at_point(
        &self,
        builder: &mut Builder<C>,
        point: Ext<<C as Config>::F, <C as Config>::EF>,
    ) -> Ext<<C as Config>::F, <C as Config>::EF> {
        let unshifted_power =
            builder.exp_power_of_2_v::<Ext<_, _>>(point * self.shift.inverse(), self.log_n.clone());
        builder.eval(unshifted_power - C::EF::ONE)
    }

    fn split_domains(
        &self,
        builder: &mut Builder<C>,
        log_num_chunks: impl Into<RVar<C::N>>,
        num_chunks: impl Into<RVar<C::N>>,
    ) -> Array<C, Self> {
        let log_num_chunks = log_num_chunks.into();
        let num_chunks = num_chunks.into();
        let log_n = builder.eval_expr(self.log_n.clone() - log_num_chunks);
        let size: Usize<_> = builder.sll(RVar::one(), log_n);

        let g_dom = self.gen();
        let g = builder.exp_power_of_2_v::<Felt<C::F>>(g_dom, log_num_chunks);

        let domain_power: Felt<_> = builder.eval(C::F::ONE);

        let domains = builder.array(num_chunks);

        builder.range(0, num_chunks).for_each(|i_vec, builder| {
            let log_n = builder.eval(log_n);
            let domain = TwoAdicMultiplicativeCosetVariable {
                log_n,
                size: size.clone(),
                shift: builder.eval(self.shift * domain_power),
                g,
            };
            // ATTENTION: here must use `builder.set_value`. `builder.set` will convert `Usize::Const`
            // to `Usize::Var` because it calls `builder.eval`.
            builder.set_value(&domains, i_vec[0], domain);
            builder.assign(&domain_power, domain_power * g_dom);
        });

        domains
    }

    fn split_domains_const(&self, builder: &mut Builder<C>, log_num_chunks: usize) -> Vec<Self> {
        let num_chunks = 1 << log_num_chunks;
        let log_n: Usize<_> =
            builder.eval(self.log_n.clone() - C::N::from_canonical_usize(log_num_chunks));
        let size: Usize<_> = builder.sll(RVar::one(), RVar::from(log_n.clone()));

        let g_dom = self.gen();
        let g = builder.exp_power_of_2_v::<Felt<C::F>>(g_dom, log_num_chunks);

        let domain_power: Felt<_> = builder.eval(C::F::ONE);
        let mut domains = vec![];

        for _ in 0..num_chunks {
            domains.push(TwoAdicMultiplicativeCosetVariable {
                log_n: log_n.clone(),
                size: size.clone(),
                shift: builder.eval(self.shift * domain_power),
                g,
            });
            builder.assign(&domain_power, domain_power * g_dom);
        }
        domains
    }

    fn create_disjoint_domain(
        &self,
        builder: &mut Builder<C>,
        log_degree: RVar<<C as Config>::N>,
        config: Option<FriConfigVariable<C>>,
    ) -> Self {
        let domain = config.unwrap().get_subgroup(builder, log_degree);
        TwoAdicMultiplicativeCosetVariable {
            log_n: domain.log_n,
            size: domain.size,
            shift: builder.eval(self.shift * C::F::GENERATOR),
            g: domain.g,
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use openvm_native_circuit::execute_program;
    use openvm_native_compiler::asm::AsmBuilder;
    use openvm_stark_backend::{
        config::{Domain, StarkGenericConfig, Val},
        p3_commit::{Pcs, PolynomialSpace},
        p3_field::PrimeField,
    };
    use openvm_stark_sdk::config::{
        baby_bear_poseidon2::{config_from_perm, default_perm, BabyBearPoseidon2Config},
        FriParameters,
    };
    use rand::{thread_rng, Rng};

    use super::*;
    use crate::utils::const_fri_config;

    pub(crate) fn domain_assertions<F: TwoAdicField + PrimeField, C: Config<N = F, F = F>>(
        builder: &mut Builder<C>,
        domain: &TwoAdicMultiplicativeCosetVariable<C>,
        domain_val: &TwoAdicMultiplicativeCoset<F>,
        zeta_val: C::EF,
    ) {
        // Assert the domain parameters are the same.
        builder.assert_var_eq(
            domain.log_n.clone(),
            F::from_canonical_usize(domain_val.log_n),
        );
        builder.assert_var_eq(
            domain.size.clone(),
            F::from_canonical_usize(1 << domain_val.log_n),
        );
        builder.assert_felt_eq(domain.shift, domain_val.shift);

        // Get a random point.
        let zeta: Ext<_, _> = builder.eval(zeta_val.cons());

        // Compare the selector values of the reference and the builder.
        let sels_expected = domain_val.selectors_at_point(zeta_val);
        let sels = domain.selectors_at_point(builder, zeta);
        builder.assert_ext_eq(sels.is_first_row, sels_expected.is_first_row.cons());
        builder.assert_ext_eq(sels.is_last_row, sels_expected.is_last_row.cons());
        builder.assert_ext_eq(sels.is_transition, sels_expected.is_transition.cons());

        let zp_val = domain_val.zp_at_point(zeta_val);
        let zp = domain.zp_at_point(builder, zeta);
        builder.assert_ext_eq(zp, zp_val.cons());
    }

    fn test_domain_impl(static_only: bool) {
        type SC = BabyBearPoseidon2Config;
        type F = Val<SC>;
        type EF = <SC as StarkGenericConfig>::Challenge;
        type Challenger = <SC as StarkGenericConfig>::Challenger;
        type ScPcs = <SC as StarkGenericConfig>::Pcs;

        let mut rng = thread_rng();
        let fri_params = FriParameters::standard_fast();
        let config = config_from_perm(&default_perm(), fri_params);
        let pcs = config.pcs();
        let natural_domain_for_degree = |degree: usize| -> Domain<SC> {
            <ScPcs as Pcs<EF, Challenger>>::natural_domain_for_degree(pcs, degree)
        };

        // Initialize a builder.
        let mut builder = AsmBuilder::<F, EF>::default();
        builder.flags.static_only = static_only;

        let config_var = const_fri_config(&mut builder, &fri_params);
        for i in 0..5 {
            let log_d_val = 10 + i;

            let log_quotient_degree = 2;

            // Initialize a reference doamin.
            let domain_val = natural_domain_for_degree(1 << log_d_val);
            let domain = builder.constant(domain_val);

            // builder.assert_felt_eq(domain.shift, domain_val.shift);
            let zeta_val = rng.gen::<EF>();
            domain_assertions(&mut builder, &domain, &domain_val, zeta_val);

            // Try a shifted domain.
            let disjoint_domain_val =
                domain_val.create_disjoint_domain(1 << (log_d_val + log_quotient_degree));
            let disjoint_domain = builder.constant(disjoint_domain_val);
            domain_assertions(
                &mut builder,
                &disjoint_domain,
                &disjoint_domain_val,
                zeta_val,
            );

            let log_degree = log_d_val + log_quotient_degree;
            let disjoint_domain_gen = domain.create_disjoint_domain(
                &mut builder,
                log_degree.into(),
                Some(config_var.clone()),
            );
            domain_assertions(
                &mut builder,
                &disjoint_domain_gen,
                &disjoint_domain_val,
                zeta_val,
            );

            // Now try splited domains
            let qc_domains_val = disjoint_domain_val.split_domains(1 << log_quotient_degree);
            for dom_val in qc_domains_val.iter() {
                let dom = builder.constant(*dom_val);
                domain_assertions(&mut builder, &dom, dom_val, zeta_val);
            }

            // Test the splitting of domains by the builder.
            let quotient_size = 1 << log_quotient_degree;
            let qc_domains =
                disjoint_domain.split_domains(&mut builder, log_quotient_degree, quotient_size);
            for (i, dom_val) in qc_domains_val.iter().enumerate() {
                let dom = builder.get(&qc_domains, i);
                domain_assertions(&mut builder, &dom, dom_val, zeta_val);
            }
        }
        builder.halt();

        let program = builder.compile_isa();
        execute_program(program, vec![]);
    }
    #[test]
    fn test_domain_static() {
        test_domain_impl(true);
    }

    #[test]
    fn test_domain_dynamic() {
        test_domain_impl(false);
    }
}
