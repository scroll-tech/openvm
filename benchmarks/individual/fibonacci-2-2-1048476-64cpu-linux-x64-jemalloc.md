| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| app_proof | <div style='text-align: right'>2</div>  | <span style="color: red">(+1,117,550 [+0.2%])</span> <div style='text-align: right'>462,554,207</div>  | <span style="color: red">(+1,842 [+0.0%])</span> <div style='text-align: right'>5,010,622</div>  | <span style="color: green">(-394.0 [-1.4%])</span> <div style='text-align: right'>28,713.0</div>  |
| leaf_aggregation | <div style='text-align: right'>2</div>  |  |  | <span style="color: green">(-70.0 [-0.5%])</span> <div style='text-align: right'>14,900.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms |
| --- | --- | --- | --- | --- |
| <span style="color: green">(-3.0 [-33.3%])</span> <div style='text-align: right'>6.0</div>  | <span style="color: red">(+8.0 [+0.7%])</span> <div style='text-align: right'>1,082.0</div>  | <span style="color: red">(+10.0 [+1.1%])</span> <div style='text-align: right'>916.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-6.0 [-1.1%])</span> <div style='text-align: right'>516.0</div>  |

| air_name | constraints | interactions | quotient_deg |
| --- | --- | --- | --- |
| ProgramAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| VmConnectorAir | <div style='text-align: right'>9</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| PersistentBoundaryAir<8> | <div style='text-align: right'>6</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| MemoryMerkleAir<8> | <div style='text-align: right'>40</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  |
| AccessAdapterAir<2> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| AccessAdapterAir<4> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| AccessAdapterAir<8> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| AccessAdapterAir<16> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| AccessAdapterAir<32> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| AccessAdapterAir<64> | <div style='text-align: right'>14</div>  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | <div style='text-align: right'>17</div>  | <div style='text-align: right'>15</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32MultAdapterAir, DivRemCoreAir<4, 8> | <div style='text-align: right'>88</div>  | <div style='text-align: right'>25</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32MultAdapterAir, MulHCoreAir<4, 8> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>24</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32MultAdapterAir, MultiplicationCoreAir<4, 8> | <div style='text-align: right'>26</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| RangeTupleCheckerAir<2> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |
| VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | <div style='text-align: right'>15</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | <div style='text-align: right'>20</div>  | <div style='text-align: right'>16</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | <div style='text-align: right'>22</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | <div style='text-align: right'>41</div>  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | <div style='text-align: right'>25</div>  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadSignExtendCoreAir<4, 8> | <div style='text-align: right'>33</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | <div style='text-align: right'>38</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | <div style='text-align: right'>90</div>  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | <div style='text-align: right'>39</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>2</div>  |
| VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | <div style='text-align: right'>43</div>  | <div style='text-align: right'>19</div>  | <div style='text-align: right'>2</div>  |
| BitwiseOperationLookupAir<8> | <div style='text-align: right'>4</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>2</div>  |
| PhantomAir | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>2</div>  |
| Poseidon2VmAir<BabyBearParameters> | <div style='text-align: right'>525</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>2</div>  |
| VariableRangeCheckerAir | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>1</div>  |

| group | segment | commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms | num_segments | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | trace_gen_time_ms | verify_program_compile_ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| app_proof | 0 |  |  | <span style="color: green">(-16.0 [-0.2%])</span> <div style='text-align: right'>6,631.0</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>1</div>  | <span style="color: green">(-394.0 [-1.4%])</span> <div style='text-align: right'>28,713.0</div>  | <div style='text-align: right'>1,072,529,432</div>  | <span style="color: red">(+1,117,550 [+0.2%])</span> <div style='text-align: right'>462,554,207</div>  | <span style="color: red">(+1,842 [+0.0%])</span> <div style='text-align: right'>5,010,622</div>  | <span style="color: green">(-122.0 [-1.4%])</span> <div style='text-align: right'>8,312.0</div>  |  |
| leaf_aggregation | 0 | <span style="color: red">(+5.0 [+10.6%])</span> <div style='text-align: right'>52.0</div>  | <span style="color: green">(-70.0 [-0.5%])</span> <div style='text-align: right'>14,900.0</div>  | <span style="color: red">(+2.0 [+0.0%])</span> <div style='text-align: right'>6,596.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: green">(-8.0 [-2.3%])</span> <div style='text-align: right'>343.0</div>  |  |  |  |  |  | <span style="color: green">(-71.0 [-0.8%])</span> <div style='text-align: right'>8,294.0</div>  | <span style="color: green">(-10.0 [-4.2%])</span> <div style='text-align: right'>228.0</div>  |

| group | chip_name | segment | rows_used |
| --- | --- | --- | --- |
| app_proof | ProgramChip | 0 | <div style='text-align: right'>106,583</div>  |
| app_proof | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| app_proof | Boundary | 0 | <span style="color: red">(+336 [+0.2%])</span> <div style='text-align: right'>171,384</div>  |
| app_proof | Merkle | 0 | <span style="color: red">(+1,382 [+0.4%])</span> <div style='text-align: right'>367,668</div>  |
| app_proof | AccessAdapter<2> | 0 | <span style="color: red">(+424 [+0.1%])</span> <div style='text-align: right'>642,352</div>  |
| app_proof | AccessAdapter<4> | 0 | <span style="color: red">(+128 [+0.0%])</span> <div style='text-align: right'>354,884</div>  |
| app_proof | AccessAdapter<8> | 0 | <div style='text-align: right'>190,130</div>  |
| app_proof | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>3</div>  |
| app_proof | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>524,288</div>  |
| app_proof | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> | 0 | <span style="color: green">(-2 [-18.2%])</span> <div style='text-align: right'>9</div>  |
| app_proof | <Rv32JalrAdapterAir,Rv32JalrCoreAir> | 0 | <span style="color: green">(-4 [-23.5%])</span> <div style='text-align: right'>13</div>  |
| app_proof | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> | 0 | <span style="color: green">(-2 [-0.0%])</span> <div style='text-align: right'>100,010</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> | 0 | <span style="color: green">(-6 [-54.5%])</span> <div style='text-align: right'>5</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> | 0 | <span style="color: green">(-3 [-0.0%])</span> <div style='text-align: right'>200,009</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> | 0 | <span style="color: green">(-29 [-50.9%])</span> <div style='text-align: right'>28</div>  |
| app_proof | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> | 0 | <span style="color: green">(-2 [-50.0%])</span> <div style='text-align: right'>2</div>  |
| app_proof | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> | 0 | <span style="color: green">(-2 [-0.0%])</span> <div style='text-align: right'>300,002</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> | 0 | <span style="color: green">(-31 [-0.0%])</span> <div style='text-align: right'>900,054</div>  |
| app_proof | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>65,536</div>  |
| app_proof | PhantomAir | 0 | <span style="color: red">(+714 [+0.3%])</span> <div style='text-align: right'>210,579</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 0 | <span style="color: red">(+1,718 [+0.3%])</span> <div style='text-align: right'>539,052</div>  |
| app_proof | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |
| app_proof | FriReducedOpeningAir | 0 | <div style='text-align: right'>144,732</div>  |
| app_proof | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> | 0 | <span style="color: green">(-26 [-0.1%])</span> <div style='text-align: right'>35,074</div>  |
| app_proof | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> | 0 | <span style="color: green">(-4,880 [-0.4%])</span> <div style='text-align: right'>1,352,716</div>  |
| app_proof | <JalNativeAdapterAir,JalCoreAir> | 0 | <span style="color: red">(+484 [+0.7%])</span> <div style='text-align: right'>74,144</div>  |
| app_proof | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> | 0 | <span style="color: red">(+802 [+0.1%])</span> <div style='text-align: right'>675,248</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> | 0 | <span style="color: red">(+4,788 [+0.4%])</span> <div style='text-align: right'>1,129,369</div>  |

| group | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- |
| app_proof |  | ADD | 0 | <span style="color: green">(-1,898 [-0.2%])</span> <div style='text-align: right'>1,150,767</div>  |
| app_proof |  | AND | 0 | <span style="color: green">(-3 [-60.0%])</span> <div style='text-align: right'>2</div>  |
| app_proof |  | AUIPC | 0 | <span style="color: green">(-2 [-18.2%])</span> <div style='text-align: right'>9</div>  |
| app_proof |  | BEQ | 0 | <span style="color: green">(-1 [-0.0%])</span> <div style='text-align: right'>100,004</div>  |
| app_proof |  | BGEU | 0 | <div style='text-align: right'>3</div>  |
| app_proof |  | BLTU | 0 | <span style="color: green">(-5 [-71.4%])</span> <div style='text-align: right'>2</div>  |
| app_proof |  | BNE | 0 | <span style="color: red">(+718 [+0.1%])</span> <div style='text-align: right'>656,692</div>  |
| app_proof |  | HINT_STOREW | 0 | <div style='text-align: right'>3</div>  |
| app_proof |  | JAL | 0 | <span style="color: green">(-1 [-0.0%])</span> <div style='text-align: right'>100,001</div>  |
| app_proof |  | JALR | 0 | <span style="color: green">(-4 [-23.5%])</span> <div style='text-align: right'>13</div>  |
| app_proof |  | LOADW | 0 | <span style="color: red">(+126 [+0.1%])</span> <div style='text-align: right'>153,258</div>  |
| app_proof |  | LUI | 0 | <span style="color: green">(-1 [-10.0%])</span> <div style='text-align: right'>9</div>  |
| app_proof |  | OR | 0 | <span style="color: green">(-3 [-75.0%])</span> <div style='text-align: right'>1</div>  |
| app_proof |  | PHANTOM | 0 | <span style="color: red">(+714 [+0.3%])</span> <div style='text-align: right'>210,579</div>  |
| app_proof |  | SLL | 0 | <span style="color: green">(-1 [-33.3%])</span> <div style='text-align: right'>2</div>  |
| app_proof |  | SLTU | 0 | <span style="color: green">(-2 [-0.0%])</span> <div style='text-align: right'>300,002</div>  |
| app_proof |  | STOREW | 0 | <span style="color: red">(+4,704 [+2.5%])</span> <div style='text-align: right'>191,087</div>  |
| app_proof |  | SUB | 0 | <span style="color: green">(-1,008 [-1.7%])</span> <div style='text-align: right'>58,273</div>  |
| app_proof |  | XOR | 0 | <span style="color: green">(-2 [-50.0%])</span> <div style='text-align: right'>2</div>  |
| app_proof |  | BBE4DIV | 0 | <div style='text-align: right'>6,268</div>  |
| app_proof |  | BBE4MUL | 0 | <span style="color: green">(-26 [-0.2%])</span> <div style='text-align: right'>11,820</div>  |
| app_proof |  | COMP_POS2 | 0 | <span style="color: red">(+84 [+0.5%])</span> <div style='text-align: right'>17,136</div>  |
| app_proof |  | DIV | 0 | <div style='text-align: right'>128</div>  |
| app_proof |  | FE4ADD | 0 | <div style='text-align: right'>13,429</div>  |
| app_proof |  | FE4SUB | 0 | <div style='text-align: right'>3,557</div>  |
| app_proof |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>5,334</div>  |
| app_proof |  | LOADW2 | 0 | <span style="color: red">(+420 [+0.1%])</span> <div style='text-align: right'>360,757</div>  |
| app_proof |  | MUL | 0 | <span style="color: green">(-1,974 [-1.4%])</span> <div style='text-align: right'>143,548</div>  |
| app_proof |  | PERM_POS2 | 0 | <span style="color: green">(-42 [-0.4%])</span> <div style='text-align: right'>10,885</div>  |
| app_proof |  | SHINTW | 0 | <span style="color: green">(-378 [-0.2%])</span> <div style='text-align: right'>244,714</div>  |
| app_proof |  | STOREW2 | 0 | <span style="color: green">(-84 [-0.0%])</span> <div style='text-align: right'>179,553</div>  |

| group | air_name | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | 0 | <span style="color: green">(-828 [-0.0%])</span> <div style='text-align: right'>32,401,620</div>  |
| app_proof | AccessAdapter<8> |  | ADD | 0 | <div style='text-align: right'>41,582</div>  |
| app_proof | Boundary |  | ADD | 0 | <div style='text-align: right'>97,840</div>  |
| app_proof | Merkle |  | ADD | 0 | <div style='text-align: right'>312,320</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | AND | 0 | <span style="color: green">(-108 [-60.0%])</span> <div style='text-align: right'>72</div>  |
| app_proof | <Rv32RdWriteAdapterAir,Rv32AuipcCoreAir> |  | AUIPC | 0 | <span style="color: green">(-42 [-18.2%])</span> <div style='text-align: right'>189</div>  |
| app_proof | AccessAdapter<8> |  | AUIPC | 0 | <div style='text-align: right'>34</div>  |
| app_proof | Boundary |  | AUIPC | 0 | <div style='text-align: right'>80</div>  |
| app_proof | Merkle |  | AUIPC | 0 | <div style='text-align: right'>3,456</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BEQ | 0 | <span style="color: green">(-26 [-0.0%])</span> <div style='text-align: right'>2,600,104</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BGEU | 0 | <div style='text-align: right'>96</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchLessThanCoreAir<4, 8>> |  | BLTU | 0 | <span style="color: green">(-160 [-71.4%])</span> <div style='text-align: right'>64</div>  |
| app_proof | <Rv32BranchAdapterAir,BranchEqualCoreAir<4>> |  | BNE | 0 | <span style="color: green">(-52 [-0.0%])</span> <div style='text-align: right'>2,600,130</div>  |
| app_proof | <Rv32HintStoreAdapterAir,Rv32HintStoreCoreAir> |  | HINT_STOREW | 0 | <div style='text-align: right'>78</div>  |
| app_proof | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | JAL | 0 | <span style="color: green">(-18 [-0.0%])</span> <div style='text-align: right'>1,800,018</div>  |
| app_proof | <Rv32JalrAdapterAir,Rv32JalrCoreAir> |  | JALR | 0 | <span style="color: green">(-112 [-23.5%])</span> <div style='text-align: right'>364</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | LOADW | 0 | <span style="color: green">(-360 [-40.9%])</span> <div style='text-align: right'>520</div>  |
| app_proof | AccessAdapter<8> |  | LOADW | 0 | <div style='text-align: right'>31,926</div>  |
| app_proof | Boundary |  | LOADW | 0 | <div style='text-align: right'>28,080</div>  |
| app_proof | Merkle |  | LOADW | 0 | <div style='text-align: right'>45,440</div>  |
| app_proof | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | 0 | <span style="color: green">(-18 [-10.0%])</span> <div style='text-align: right'>162</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | 0 | <span style="color: green">(-108 [-75.0%])</span> <div style='text-align: right'>36</div>  |
| app_proof | PhantomAir |  | PHANTOM | 0 | <span style="color: red">(+4,284 [+0.3%])</span> <div style='text-align: right'>1,263,474</div>  |
| app_proof | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | 0 | <span style="color: green">(-53 [-33.3%])</span> <div style='text-align: right'>106</div>  |
| app_proof | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | 0 | <span style="color: green">(-74 [-0.0%])</span> <div style='text-align: right'>11,100,074</div>  |
| app_proof | AccessAdapter<8> |  | SLTU | 0 | <div style='text-align: right'>34</div>  |
| app_proof | Boundary |  | SLTU | 0 | <div style='text-align: right'>80</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | 0 | <span style="color: green">(-520 [-46.4%])</span> <div style='text-align: right'>600</div>  |
| app_proof | AccessAdapter<8> |  | STOREW | 0 | <span style="color: red">(+4,284 [+2.3%])</span> <div style='text-align: right'>191,029</div>  |
| app_proof | Boundary |  | STOREW | 0 | <span style="color: red">(+10,080 [+2.3%])</span> <div style='text-align: right'>449,480</div>  |
| app_proof | Merkle |  | STOREW | 0 | <span style="color: red">(+57,664 [+2.1%])</span> <div style='text-align: right'>2,800,064</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | 0 | <div style='text-align: right'>144</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | 0 | <span style="color: green">(-72 [-50.0%])</span> <div style='text-align: right'>72</div>  |
| app_proof | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | ADD | 0 | <span style="color: green">(-56,940 [-0.2%])</span> <div style='text-align: right'>34,523,010</div>  |
| app_proof | AccessAdapter<2> |  | ADD | 0 | <span style="color: red">(+484 [+0.2%])</span> <div style='text-align: right'>277,827</div>  |
| app_proof | AccessAdapter<4> |  | ADD | 0 | <span style="color: red">(+286 [+0.2%])</span> <div style='text-align: right'>164,333</div>  |
| app_proof | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4DIV | 0 | <div style='text-align: right'>250,720</div>  |
| app_proof | AccessAdapter<2> |  | BBE4DIV | 0 | <div style='text-align: right'>120,692</div>  |
| app_proof | AccessAdapter<4> |  | BBE4DIV | 0 | <div style='text-align: right'>71,318</div>  |
| app_proof | AccessAdapter<8> |  | BBE4DIV | 0 | <div style='text-align: right'>34</div>  |
| app_proof | Boundary |  | BBE4DIV | 0 | <div style='text-align: right'>80</div>  |
| app_proof | Merkle |  | BBE4DIV | 0 | <div style='text-align: right'>384</div>  |
| app_proof | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | BBE4MUL | 0 | <span style="color: green">(-1,040 [-0.2%])</span> <div style='text-align: right'>472,800</div>  |
| app_proof | AccessAdapter<2> |  | BBE4MUL | 0 | <span style="color: red">(+484 [+0.2%])</span> <div style='text-align: right'>234,564</div>  |
| app_proof | AccessAdapter<4> |  | BBE4MUL | 0 | <span style="color: red">(+286 [+0.2%])</span> <div style='text-align: right'>138,606</div>  |
| app_proof | AccessAdapter<8> |  | BBE4MUL | 0 | <div style='text-align: right'>34,221</div>  |
| app_proof | Boundary |  | BBE4MUL | 0 | <div style='text-align: right'>80,520</div>  |
| app_proof | Merkle |  | BBE4MUL | 0 | <div style='text-align: right'>31,424</div>  |
| app_proof | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BEQ | 0 | <span style="color: red">(+1,932 [+0.5%])</span> <div style='text-align: right'>426,788</div>  |
| app_proof | <BranchNativeAdapterAir,BranchEqualCoreAir<1>> |  | BNE | 0 | <span style="color: red">(+16,514 [+0.1%])</span> <div style='text-align: right'>15,103,916</div>  |
| app_proof | AccessAdapter<2> |  | BNE | 0 | <div style='text-align: right'>1,386</div>  |
| app_proof | AccessAdapter<4> |  | BNE | 0 | <div style='text-align: right'>819</div>  |
| app_proof | AccessAdapter<2> |  | COMP_POS2 | 0 | <span style="color: green">(-1,848 [-0.3%])</span> <div style='text-align: right'>687,456</div>  |
| app_proof | AccessAdapter<4> |  | COMP_POS2 | 0 | <span style="color: green">(-1,092 [-0.3%])</span> <div style='text-align: right'>406,224</div>  |
| app_proof | AccessAdapter<8> |  | COMP_POS2 | 0 | <span style="color: green">(-714 [-0.3%])</span> <div style='text-align: right'>265,608</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> |  | COMP_POS2 | 0 | <span style="color: red">(+46,956 [+0.5%])</span> <div style='text-align: right'>9,579,024</div>  |
| app_proof | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | DIV | 0 | <div style='text-align: right'>3,840</div>  |
| app_proof | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4ADD | 0 | <div style='text-align: right'>537,160</div>  |
| app_proof | AccessAdapter<2> |  | FE4ADD | 0 | <div style='text-align: right'>189,288</div>  |
| app_proof | AccessAdapter<4> |  | FE4ADD | 0 | <div style='text-align: right'>111,852</div>  |
| app_proof | AccessAdapter<8> |  | FE4ADD | 0 | <div style='text-align: right'>27,115</div>  |
| app_proof | Boundary |  | FE4ADD | 0 | <div style='text-align: right'>63,800</div>  |
| app_proof | Merkle |  | FE4ADD | 0 | <div style='text-align: right'>58,752</div>  |
| app_proof | <NativeVectorizedAdapterAir<4>,FieldExtensionCoreAir> |  | FE4SUB | 0 | <div style='text-align: right'>142,280</div>  |
| app_proof | AccessAdapter<2> |  | FE4SUB | 0 | <div style='text-align: right'>112,442</div>  |
| app_proof | AccessAdapter<4> |  | FE4SUB | 0 | <div style='text-align: right'>66,443</div>  |
| app_proof | AccessAdapter<8> |  | FE4SUB | 0 | <div style='text-align: right'>8,381</div>  |
| app_proof | Boundary |  | FE4SUB | 0 | <div style='text-align: right'>19,720</div>  |
| app_proof | Merkle |  | FE4SUB | 0 | <div style='text-align: right'>1,472</div>  |
| app_proof | AccessAdapter<2> |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>151,580</div>  |
| app_proof | AccessAdapter<4> |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>89,570</div>  |
| app_proof | FriReducedOpeningAir |  | FRI_REDUCED_OPENING | 0 | <div style='text-align: right'>9,262,848</div>  |
| app_proof | <JalNativeAdapterAir,JalCoreAir> |  | JAL | 0 | <span style="color: red">(+4,840 [+0.7%])</span> <div style='text-align: right'>741,440</div>  |
| app_proof | AccessAdapter<2> |  | JAL | 0 | <div style='text-align: right'>418</div>  |
| app_proof | AccessAdapter<4> |  | JAL | 0 | <div style='text-align: right'>494</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LOADW | 0 | <span style="color: red">(+5,166 [+0.1%])</span> <div style='text-align: right'>6,283,578</div>  |
| app_proof | AccessAdapter<2> |  | LOADW | 0 | <div style='text-align: right'>294,206</div>  |
| app_proof | AccessAdapter<4> |  | LOADW | 0 | <div style='text-align: right'>143,728</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | LOADW2 | 0 | <span style="color: red">(+17,220 [+0.1%])</span> <div style='text-align: right'>14,791,037</div>  |
| app_proof | AccessAdapter<2> |  | LOADW2 | 0 | <div style='text-align: right'>58,025</div>  |
| app_proof | AccessAdapter<4> |  | LOADW2 | 0 | <div style='text-align: right'>34,424</div>  |
| app_proof | AccessAdapter<8> |  | LOADW2 | 0 | <div style='text-align: right'>1,292</div>  |
| app_proof | Boundary |  | LOADW2 | 0 | <div style='text-align: right'>1,880</div>  |
| app_proof | Merkle |  | LOADW2 | 0 | <div style='text-align: right'>2,816</div>  |
| app_proof | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | MUL | 0 | <span style="color: green">(-59,220 [-1.4%])</span> <div style='text-align: right'>4,306,440</div>  |
| app_proof | AccessAdapter<2> |  | MUL | 0 | <div style='text-align: right'>32,956</div>  |
| app_proof | AccessAdapter<4> |  | MUL | 0 | <div style='text-align: right'>23,530</div>  |
| app_proof | AccessAdapter<8> |  | MUL | 0 | <div style='text-align: right'>11,407</div>  |
| app_proof | Boundary |  | MUL | 0 | <div style='text-align: right'>26,840</div>  |
| app_proof | Merkle |  | MUL | 0 | <div style='text-align: right'>43,648</div>  |
| app_proof | AccessAdapter<2> |  | PERM_POS2 | 0 | <span style="color: red">(+4,620 [+0.8%])</span> <div style='text-align: right'>583,396</div>  |
| app_proof | AccessAdapter<4> |  | PERM_POS2 | 0 | <span style="color: red">(+2,730 [+0.8%])</span> <div style='text-align: right'>346,372</div>  |
| app_proof | AccessAdapter<8> |  | PERM_POS2 | 0 | <span style="color: red">(+1,428 [+0.6%])</span> <div style='text-align: right'>230,758</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> |  | PERM_POS2 | 0 | <span style="color: green">(-23,478 [-0.4%])</span> <div style='text-align: right'>6,084,715</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | SHINTW | 0 | <span style="color: green">(-15,498 [-0.2%])</span> <div style='text-align: right'>10,033,274</div>  |
| app_proof | AccessAdapter<2> |  | SHINTW | 0 | <span style="color: green">(-2,310 [-0.2%])</span> <div style='text-align: right'>1,488,707</div>  |
| app_proof | AccessAdapter<4> |  | SHINTW | 0 | <span style="color: green">(-1,638 [-0.2%])</span> <div style='text-align: right'>1,049,516</div>  |
| app_proof | AccessAdapter<8> |  | SHINTW | 0 | <span style="color: green">(-1,428 [-0.2%])</span> <div style='text-align: right'>932,960</div>  |
| app_proof | Boundary |  | SHINTW | 0 | <span style="color: green">(-3,360 [-0.2%])</span> <div style='text-align: right'>2,195,200</div>  |
| app_proof | Merkle |  | SHINTW | 0 | <span style="color: green">(-13,440 [-0.2%])</span> <div style='text-align: right'>7,685,696</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW | 0 | <span style="color: red">(+192,864 [+2.5%])</span> <div style='text-align: right'>7,834,567</div>  |
| app_proof | AccessAdapter<2> |  | STOREW | 0 | <span style="color: red">(+11,088 [+2.6%])</span> <div style='text-align: right'>438,020</div>  |
| app_proof | AccessAdapter<4> |  | STOREW | 0 | <span style="color: red">(+6,552 [+2.5%])</span> <div style='text-align: right'>273,286</div>  |
| app_proof | <NativeLoadStoreAdapterAir<1>,NativeLoadStoreCoreAir<1>> |  | STOREW2 | 0 | <span style="color: green">(-3,444 [-0.0%])</span> <div style='text-align: right'>7,361,673</div>  |
| app_proof | AccessAdapter<2> |  | STOREW2 | 0 | <span style="color: green">(-6,930 [-0.8%])</span> <div style='text-align: right'>867,229</div>  |
| app_proof | AccessAdapter<4> |  | STOREW2 | 0 | <span style="color: green">(-4,368 [-0.8%])</span> <div style='text-align: right'>529,828</div>  |
| app_proof | AccessAdapter<8> |  | STOREW2 | 0 | <span style="color: green">(-2,856 [-0.9%])</span> <div style='text-align: right'>314,211</div>  |
| app_proof | Boundary |  | STOREW2 | 0 | <div style='text-align: right'>412,600</div>  |
| app_proof | Merkle |  | STOREW2 | 0 | <div style='text-align: right'>700,608</div>  |
| app_proof | <NativeAdapterAir<2, 1>,FieldArithmeticCoreAir> |  | SUB | 0 | <span style="color: green">(-30,240 [-1.7%])</span> <div style='text-align: right'>1,748,190</div>  |
| app_proof | AccessAdapter<2> |  | SUB | 0 | <div style='text-align: right'>74,162</div>  |
| app_proof | AccessAdapter<4> |  | SUB | 0 | <div style='text-align: right'>86,788</div>  |
| app_proof | AccessAdapter<8> |  | SUB | 0 | <div style='text-align: right'>21,947</div>  |
| app_proof | Boundary |  | SUB | 0 | <div style='text-align: right'>51,640</div>  |
| app_proof | Merkle |  | SUB | 0 | <div style='text-align: right'>82,688</div>  |

| group | execute_time_ms | fri.log_blowup | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| app_proof | <span style="color: green">(-34.0 [-3.7%])</span> <div style='text-align: right'>878.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <span style="color: red">(+1,117,550 [+0.2%])</span> <div style='text-align: right'>462,554,207</div>  | <span style="color: red">(+1,842 [+0.0%])</span> <div style='text-align: right'>5,010,622</div>  | <span style="color: green">(-394.0 [-1.4%])</span> <div style='text-align: right'>28,713.0</div>  |
| leaf_aggregation |  | <div style='text-align: right'>2</div>  |  |  |  | <span style="color: green">(-70.0 [-0.5%])</span> <div style='text-align: right'>14,900.0</div>  |

| group | air_name | segment | cells | constraints | interactions | main_cols | perm_cols | prep_cols | quotient_deg | rows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| app_proof | ProgramAir | 0 | <div style='text-align: right'>2,359,296</div>  |  |  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  |  |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  |  | <div style='text-align: right'>2</div>  |
| app_proof | PersistentBoundaryAir<8> | 0 | <div style='text-align: right'>7,340,032</div>  |  |  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| app_proof | MemoryMerkleAir<8> | 0 | <div style='text-align: right'>23,068,672</div>  |  |  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| app_proof | AccessAdapterAir<8> | 0 | <div style='text-align: right'>8,650,752</div>  |  |  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| app_proof | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>248</div>  |  |  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  |  | <div style='text-align: right'>4</div>  |
| app_proof | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>784</div>  |  |  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  |  | <div style='text-align: right'>16</div>  |
| app_proof | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <span style="color: green">(-1,024 [-50.0%])</span> <div style='text-align: right'>1,024</div>  |  |  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  |  | <span style="color: green">(-16 [-50.0%])</span> <div style='text-align: right'>16</div>  |
| app_proof | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>8,126,464</div>  |  |  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <span style="color: green">(-704 [-50.0%])</span> <div style='text-align: right'>704</div>  |  |  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  |  | <span style="color: green">(-8 [-50.0%])</span> <div style='text-align: right'>8</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>19,398,656</div>  |  |  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| app_proof | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <span style="color: green">(-3,584 [-50.0%])</span> <div style='text-align: right'>3,584</div>  |  |  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  |  | <span style="color: green">(-32 [-50.0%])</span> <div style='text-align: right'>32</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <span style="color: green">(-210 [-50.0%])</span> <div style='text-align: right'>210</div>  |  |  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  |  | <span style="color: green">(-2 [-50.0%])</span> <div style='text-align: right'>2</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>40,370,176</div>  |  |  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>121,634,816</div>  |  |  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  |  | <div style='text-align: right'>1,048,576</div>  |
| app_proof | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  |  |  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  |  | <div style='text-align: right'>65,536</div>  |
| app_proof | PhantomAir | 0 | <div style='text-align: right'>3,670,016</div>  |  |  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 0 | <div style='text-align: right'>623,902,720</div>  |  |  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  |  | <div style='text-align: right'>1,048,576</div>  |
| app_proof | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  |  |  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  |  | <div style='text-align: right'>262,144</div>  |
| leaf_aggregation | ProgramAir | 0 |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  |  |  |  | <div style='text-align: right'>1</div>  |  |
| leaf_aggregation | VmConnectorAir | 0 |  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | PersistentBoundaryAir<8> | 0 |  | <div style='text-align: right'>5</div>  | <div style='text-align: right'>3</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | MemoryMerkleAir<8> | 0 |  | <div style='text-align: right'>38</div>  | <div style='text-align: right'>4</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | AccessAdapterAir<2> | 0 |  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | AccessAdapterAir<4> | 0 |  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | AccessAdapterAir<8> | 0 |  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>5</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | Poseidon2VmAir<BabyBearParameters> | 0 |  | <div style='text-align: right'>517</div>  | <div style='text-align: right'>32</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | FriReducedOpeningAir | 0 |  | <div style='text-align: right'>59</div>  | <div style='text-align: right'>35</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 |  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 |  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>15</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 |  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>7</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 |  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>11</div>  |  |  |  | <div style='text-align: right'>2</div>  |  |
| leaf_aggregation | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 |  | <div style='text-align: right'>31</div>  | <div style='text-align: right'>19</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | PhantomAir | 0 |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>3</div>  |  |  |  | <div style='text-align: right'>4</div>  |  |
| leaf_aggregation | VariableRangeCheckerAir | 0 |  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>1</div>  |  |  |  | <div style='text-align: right'>1</div>  |  |
| app_proof | AccessAdapterAir<2> | 0 | <div style='text-align: right'>28,311,552</div>  |  |  | <div style='text-align: right'>11</div>  | <div style='text-align: right'>16</div>  |  |  | <div style='text-align: right'>1,048,576</div>  |
| app_proof | AccessAdapterAir<4> | 0 | <div style='text-align: right'>15,204,352</div>  |  |  | <div style='text-align: right'>13</div>  | <div style='text-align: right'>16</div>  |  |  | <div style='text-align: right'>524,288</div>  |
| app_proof | FriReducedOpeningAir | 0 | <div style='text-align: right'>36,700,160</div>  |  |  | <div style='text-align: right'>64</div>  | <div style='text-align: right'>76</div>  |  |  | <div style='text-align: right'>262,144</div>  |
| app_proof | VmAirWrapper<NativeVectorizedAdapterAir<4>, FieldExtensionCoreAir> | 0 | <div style='text-align: right'>3,932,160</div>  |  |  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>20</div>  |  |  | <div style='text-align: right'>65,536</div>  |
| app_proof | VmAirWrapper<NativeAdapterAir<2, 1>, FieldArithmeticCoreAir> | 0 | <div style='text-align: right'>104,857,600</div>  |  |  | <div style='text-align: right'>30</div>  | <div style='text-align: right'>20</div>  |  |  | <div style='text-align: right'>2,097,152</div>  |
| app_proof | VmAirWrapper<JalNativeAdapterAir, JalCoreAir> | 0 | <div style='text-align: right'>2,883,584</div>  |  |  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>12</div>  |  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | VmAirWrapper<BranchNativeAdapterAir, BranchEqualCoreAir<1> | 0 | <div style='text-align: right'>53,477,376</div>  |  |  | <div style='text-align: right'>23</div>  | <div style='text-align: right'>28</div>  |  |  | <div style='text-align: right'>1,048,576</div>  |
| app_proof | VmAirWrapper<NativeLoadStoreAdapterAir<1>, NativeLoadStoreCoreAir<1> | 0 | <div style='text-align: right'>136,314,880</div>  |  |  | <div style='text-align: right'>41</div>  | <div style='text-align: right'>24</div>  |  |  | <div style='text-align: right'>2,097,152</div>  |

| segment | trace_gen_time_ms |
| --- | --- |
| 0 | <span style="color: green">(-2.0 [-1.2%])</span> <div style='text-align: right'>165.0</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/02fc1b3cb39968d34c9ca643a5909ab229342a5e/fibonacci-2-2-1048476-64cpu-linux-x64-jemalloc-app_proof.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/02fc1b3cb39968d34c9ca643a5909ab229342a5e/fibonacci-2-2-1048476-64cpu-linux-x64-jemalloc-app_proof.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/02fc1b3cb39968d34c9ca643a5909ab229342a5e/fibonacci-2-2-1048476-64cpu-linux-x64-jemalloc-app_proof.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/02fc1b3cb39968d34c9ca643a5909ab229342a5e/fibonacci-2-2-1048476-64cpu-linux-x64-jemalloc-app_proof.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/02fc1b3cb39968d34c9ca643a5909ab229342a5e/fibonacci-2-2-1048476-64cpu-linux-x64-jemalloc-app_proof.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/02fc1b3cb39968d34c9ca643a5909ab229342a5e/fibonacci-2-2-1048476-64cpu-linux-x64-jemalloc-app_proof.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/02fc1b3cb39968d34c9ca643a5909ab229342a5e/fibonacci-2-2-1048476-64cpu-linux-x64-jemalloc-app_proof.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/02fc1b3cb39968d34c9ca643a5909ab229342a5e/fibonacci-2-2-1048476-64cpu-linux-x64-jemalloc-app_proof.dsl_ir.opcode.frequency.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/02fc1b3cb39968d34c9ca643a5909ab229342a5e/fibonacci-2-2-1048476-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/02fc1b3cb39968d34c9ca643a5909ab229342a5e/fibonacci-2-2-1048476-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/02fc1b3cb39968d34c9ca643a5909ab229342a5e/fibonacci-2-2-1048476-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/02fc1b3cb39968d34c9ca643a5909ab229342a5e/fibonacci-2-2-1048476-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/02fc1b3cb39968d34c9ca643a5909ab229342a5e/fibonacci-2-2-1048476-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/02fc1b3cb39968d34c9ca643a5909ab229342a5e/fibonacci-2-2-1048476-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/02fc1b3cb39968d34c9ca643a5909ab229342a5e/fibonacci-2-2-1048476-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/02fc1b3cb39968d34c9ca643a5909ab229342a5e/fibonacci-2-2-1048476-64cpu-linux-x64-jemalloc-leaf_aggregation.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/02fc1b3cb39968d34c9ca643a5909ab229342a5e

Max Segment Length: 1048476

Instance Type: 64cpu-linux-x64

Memory Allocator: jemalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12272017165)
