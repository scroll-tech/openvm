| group | fri.log_blowup | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- |
| app_proof | <div style='text-align: right'>2</div>  | <span style="color: green">(-409,832,473 [-88.8%])</span> <div style='text-align: right'>51,612,244</div>  | <span style="color: green">(-3,509,253 [-70.1%])</span> <div style='text-align: right'>1,500,137</div>  | <span style="color: green">(-21,959.0 [-81.0%])</span> <div style='text-align: right'>5,154.0</div>  |


<details>
<summary>Detailed Metrics</summary>

| commit_exe_time_ms | execute_and_trace_gen_time_ms | execute_time_ms | fri.log_blowup | keygen_time_ms |
| --- | --- | --- | --- | --- |
| <div style='text-align: right'>4.0</div>  | <span style="color: green">(-28.0 [-2.3%])</span> <div style='text-align: right'>1,199.0</div>  | <span style="color: green">(-30.0 [-3.1%])</span> <div style='text-align: right'>932.0</div>  | <div style='text-align: right'>2</div>  | <span style="color: red">(+2.0 [+0.4%])</span> <div style='text-align: right'>487.0</div>  |

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

| group | segment | stark_prove_excluding_trace_time_ms | total_cells | total_cells_used | total_cycles | trace_gen_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| app_proof | 0 | <span style="color: green">(-21,959.0 [-81.0%])</span> <div style='text-align: right'>5,154.0</div>  | <span style="color: green">(-874,833,402 [-81.6%])</span> <div style='text-align: right'>197,696,030</div>  | <span style="color: green">(-409,832,473 [-88.8%])</span> <div style='text-align: right'>51,612,244</div>  | <span style="color: green">(-3,509,253 [-70.1%])</span> <div style='text-align: right'>1,500,137</div>  | <span style="color: green">(-8,975.0 [-97.4%])</span> <div style='text-align: right'>241.0</div>  |

| group | chip_name | segment | rows_used |
| --- | --- | --- | --- |
| app_proof | ProgramChip | 0 | <span style="color: green">(-103,248 [-96.9%])</span> <div style='text-align: right'>3,335</div>  |
| app_proof | VmConnectorAir | 0 | <div style='text-align: right'>2</div>  |
| app_proof | Boundary | 0 | <span style="color: green">(-171,012 [-100.0%])</span> <div style='text-align: right'>36</div>  |
| app_proof | Merkle | 0 | <span style="color: green">(-366,006 [-99.9%])</span> <div style='text-align: right'>280</div>  |
| app_proof | AccessAdapter<8> | 0 | <span style="color: green">(-190,094 [-100.0%])</span> <div style='text-align: right'>36</div>  |
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
| app_proof | PhantomAir | 0 | <span style="color: green">(-209,863 [-100.0%])</span> <div style='text-align: right'>2</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 0 | <span style="color: green">(-537,018 [-99.9%])</span> <div style='text-align: right'>316</div>  |
| app_proof | VariableRangeCheckerAir | 0 | <div style='text-align: right'>262,144</div>  |

| group | dsl_ir | opcode | segment | frequency |
| --- | --- | --- | --- | --- |
| app_proof |  | ADD | 0 | <span style="color: green">(-252,620 [-21.9%])</span> <div style='text-align: right'>900,045</div>  |
| app_proof |  | AND | 0 | <span style="color: green">(-3 [-60.0%])</span> <div style='text-align: right'>2</div>  |
| app_proof |  | AUIPC | 0 | <span style="color: green">(-2 [-18.2%])</span> <div style='text-align: right'>9</div>  |
| app_proof |  | BEQ | 0 | <span style="color: green">(-1 [-0.0%])</span> <div style='text-align: right'>100,004</div>  |
| app_proof |  | BGEU | 0 | <div style='text-align: right'>3</div>  |
| app_proof |  | BLTU | 0 | <span style="color: green">(-5 [-71.4%])</span> <div style='text-align: right'>2</div>  |
| app_proof |  | BNE | 0 | <span style="color: green">(-555,969 [-84.8%])</span> <div style='text-align: right'>100,005</div>  |
| app_proof |  | HINT_STOREW | 0 | <div style='text-align: right'>3</div>  |
| app_proof |  | JAL | 0 | <span style="color: green">(-1 [-0.0%])</span> <div style='text-align: right'>100,001</div>  |
| app_proof |  | JALR | 0 | <span style="color: green">(-4 [-23.5%])</span> <div style='text-align: right'>13</div>  |
| app_proof |  | LOADW | 0 | <span style="color: green">(-153,119 [-100.0%])</span> <div style='text-align: right'>13</div>  |
| app_proof |  | LUI | 0 | <span style="color: green">(-1 [-10.0%])</span> <div style='text-align: right'>9</div>  |
| app_proof |  | OR | 0 | <span style="color: green">(-3 [-75.0%])</span> <div style='text-align: right'>1</div>  |
| app_proof |  | PHANTOM | 0 | <span style="color: green">(-209,863 [-100.0%])</span> <div style='text-align: right'>2</div>  |
| app_proof |  | SLL | 0 | <span style="color: green">(-1 [-33.3%])</span> <div style='text-align: right'>2</div>  |
| app_proof |  | SLTU | 0 | <span style="color: green">(-2 [-0.0%])</span> <div style='text-align: right'>300,002</div>  |
| app_proof |  | STOREW | 0 | <span style="color: green">(-186,368 [-100.0%])</span> <div style='text-align: right'>15</div>  |
| app_proof |  | SUB | 0 | <span style="color: green">(-59,277 [-100.0%])</span> <div style='text-align: right'>4</div>  |
| app_proof |  | XOR | 0 | <span style="color: green">(-2 [-50.0%])</span> <div style='text-align: right'>2</div>  |

| group | air_name | dsl_ir | opcode | segment | cells_used |
| --- | --- | --- | --- | --- | --- |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | ADD | 0 | <span style="color: green">(-828 [-0.0%])</span> <div style='text-align: right'>32,401,620</div>  |
| app_proof | AccessAdapter<8> |  | ADD | 0 | <span style="color: green">(-41,514 [-99.8%])</span> <div style='text-align: right'>68</div>  |
| app_proof | Boundary |  | ADD | 0 | <span style="color: green">(-97,680 [-99.8%])</span> <div style='text-align: right'>160</div>  |
| app_proof | Merkle |  | ADD | 0 | <span style="color: green">(-312,000 [-99.9%])</span> <div style='text-align: right'>320</div>  |
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
| app_proof | AccessAdapter<8> |  | LOADW | 0 | <span style="color: green">(-31,892 [-99.9%])</span> <div style='text-align: right'>34</div>  |
| app_proof | Boundary |  | LOADW | 0 | <span style="color: green">(-28,000 [-99.7%])</span> <div style='text-align: right'>80</div>  |
| app_proof | Merkle |  | LOADW | 0 | <span style="color: green">(-43,136 [-94.9%])</span> <div style='text-align: right'>2,304</div>  |
| app_proof | <Rv32CondRdWriteAdapterAir,Rv32JalLuiCoreAir> |  | LUI | 0 | <span style="color: green">(-18 [-10.0%])</span> <div style='text-align: right'>162</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | OR | 0 | <span style="color: green">(-108 [-75.0%])</span> <div style='text-align: right'>36</div>  |
| app_proof | PhantomAir |  | PHANTOM | 0 | <span style="color: green">(-1,259,178 [-100.0%])</span> <div style='text-align: right'>12</div>  |
| app_proof | <Rv32BaseAluAdapterAir,ShiftCoreAir<4, 8>> |  | SLL | 0 | <span style="color: green">(-53 [-33.3%])</span> <div style='text-align: right'>106</div>  |
| app_proof | <Rv32BaseAluAdapterAir,LessThanCoreAir<4, 8>> |  | SLTU | 0 | <span style="color: green">(-74 [-0.0%])</span> <div style='text-align: right'>11,100,074</div>  |
| app_proof | AccessAdapter<8> |  | SLTU | 0 | <div style='text-align: right'>34</div>  |
| app_proof | Boundary |  | SLTU | 0 | <div style='text-align: right'>80</div>  |
| app_proof | <Rv32LoadStoreAdapterAir,LoadStoreCoreAir<4>> |  | STOREW | 0 | <span style="color: green">(-520 [-46.4%])</span> <div style='text-align: right'>600</div>  |
| app_proof | AccessAdapter<8> |  | STOREW | 0 | <span style="color: green">(-186,609 [-99.9%])</span> <div style='text-align: right'>136</div>  |
| app_proof | Boundary |  | STOREW | 0 | <span style="color: green">(-439,080 [-99.9%])</span> <div style='text-align: right'>320</div>  |
| app_proof | Merkle |  | STOREW | 0 | <span style="color: green">(-2,739,584 [-99.9%])</span> <div style='text-align: right'>2,816</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | SUB | 0 | <div style='text-align: right'>144</div>  |
| app_proof | <Rv32BaseAluAdapterAir,BaseAluCoreAir<4, 8>> |  | XOR | 0 | <span style="color: green">(-72 [-50.0%])</span> <div style='text-align: right'>72</div>  |

| group | execute_time_ms | fri.log_blowup | num_segments | total_cells_used | total_cycles | total_proof_time_ms |
| --- | --- | --- | --- | --- | --- | --- |
| app_proof | <span style="color: green">(-38.0 [-3.9%])</span> <div style='text-align: right'>945.0</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>1</div>  | <span style="color: green">(-409,832,473 [-88.8%])</span> <div style='text-align: right'>51,612,244</div>  | <span style="color: green">(-3,509,253 [-70.1%])</span> <div style='text-align: right'>1,500,137</div>  | <span style="color: green">(-21,959.0 [-81.0%])</span> <div style='text-align: right'>5,154.0</div>  |

| group | air_name | segment | cells | main_cols | perm_cols | prep_cols | rows |
| --- | --- | --- | --- | --- | --- | --- | --- |
| app_proof | ProgramAir | 0 | <span style="color: green">(-2,285,568 [-96.9%])</span> <div style='text-align: right'>73,728</div>  | <div style='text-align: right'>10</div>  | <div style='text-align: right'>8</div>  |  | <span style="color: green">(-126,976 [-96.9%])</span> <div style='text-align: right'>4,096</div>  |
| app_proof | VmConnectorAir | 0 | <div style='text-align: right'>32</div>  | <div style='text-align: right'>4</div>  | <div style='text-align: right'>12</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>2</div>  |
| app_proof | PersistentBoundaryAir<8> | 0 | <span style="color: green">(-7,337,984 [-100.0%])</span> <div style='text-align: right'>2,048</div>  | <div style='text-align: right'>20</div>  | <div style='text-align: right'>12</div>  |  | <span style="color: green">(-262,080 [-100.0%])</span> <div style='text-align: right'>64</div>  |
| app_proof | MemoryMerkleAir<8> | 0 | <span style="color: green">(-23,042,048 [-99.9%])</span> <div style='text-align: right'>26,624</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>20</div>  |  | <span style="color: green">(-523,776 [-99.9%])</span> <div style='text-align: right'>512</div>  |
| app_proof | AccessAdapterAir<8> | 0 | <span style="color: green">(-8,648,128 [-100.0%])</span> <div style='text-align: right'>2,624</div>  | <div style='text-align: right'>17</div>  | <div style='text-align: right'>24</div>  |  | <span style="color: green">(-262,080 [-100.0%])</span> <div style='text-align: right'>64</div>  |
| app_proof | VmAirWrapper<Rv32HintStoreAdapterAir, Rv32HintStoreCoreAir> | 0 | <div style='text-align: right'>248</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>36</div>  |  | <div style='text-align: right'>4</div>  |
| app_proof | RangeTupleCheckerAir<2> | 0 | <div style='text-align: right'>4,718,592</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32RdWriteAdapterAir, Rv32AuipcCoreAir> | 0 | <div style='text-align: right'>784</div>  | <div style='text-align: right'>21</div>  | <div style='text-align: right'>28</div>  |  | <div style='text-align: right'>16</div>  |
| app_proof | VmAirWrapper<Rv32JalrAdapterAir, Rv32JalrCoreAir> | 0 | <span style="color: green">(-1,024 [-50.0%])</span> <div style='text-align: right'>1,024</div>  | <div style='text-align: right'>28</div>  | <div style='text-align: right'>36</div>  |  | <span style="color: green">(-16 [-50.0%])</span> <div style='text-align: right'>16</div>  |
| app_proof | VmAirWrapper<Rv32CondRdWriteAdapterAir, Rv32JalLuiCoreAir> | 0 | <div style='text-align: right'>8,126,464</div>  | <div style='text-align: right'>18</div>  | <div style='text-align: right'>44</div>  |  | <div style='text-align: right'>131,072</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchLessThanCoreAir<4, 8> | 0 | <span style="color: green">(-704 [-50.0%])</span> <div style='text-align: right'>704</div>  | <div style='text-align: right'>32</div>  | <div style='text-align: right'>56</div>  |  | <span style="color: green">(-8 [-50.0%])</span> <div style='text-align: right'>8</div>  |
| app_proof | VmAirWrapper<Rv32BranchAdapterAir, BranchEqualCoreAir<4> | 0 | <div style='text-align: right'>19,398,656</div>  | <div style='text-align: right'>26</div>  | <div style='text-align: right'>48</div>  |  | <div style='text-align: right'>262,144</div>  |
| app_proof | VmAirWrapper<Rv32LoadStoreAdapterAir, LoadStoreCoreAir<4> | 0 | <span style="color: green">(-3,584 [-50.0%])</span> <div style='text-align: right'>3,584</div>  | <div style='text-align: right'>40</div>  | <div style='text-align: right'>72</div>  |  | <span style="color: green">(-32 [-50.0%])</span> <div style='text-align: right'>32</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, ShiftCoreAir<4, 8> | 0 | <span style="color: green">(-210 [-50.0%])</span> <div style='text-align: right'>210</div>  | <div style='text-align: right'>53</div>  | <div style='text-align: right'>52</div>  |  | <span style="color: green">(-2 [-50.0%])</span> <div style='text-align: right'>2</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, LessThanCoreAir<4, 8> | 0 | <div style='text-align: right'>40,370,176</div>  | <div style='text-align: right'>37</div>  | <div style='text-align: right'>40</div>  |  | <div style='text-align: right'>524,288</div>  |
| app_proof | VmAirWrapper<Rv32BaseAluAdapterAir, BaseAluCoreAir<4, 8> | 0 | <div style='text-align: right'>121,634,816</div>  | <div style='text-align: right'>36</div>  | <div style='text-align: right'>80</div>  |  | <div style='text-align: right'>1,048,576</div>  |
| app_proof | BitwiseOperationLookupAir<8> | 0 | <div style='text-align: right'>655,360</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>3</div>  | <div style='text-align: right'>65,536</div>  |
| app_proof | PhantomAir | 0 | <span style="color: green">(-3,669,980 [-100.0%])</span> <div style='text-align: right'>36</div>  | <div style='text-align: right'>6</div>  | <div style='text-align: right'>12</div>  |  | <span style="color: green">(-262,142 [-100.0%])</span> <div style='text-align: right'>2</div>  |
| app_proof | Poseidon2VmAir<BabyBearParameters> | 0 | <span style="color: green">(-623,581,696 [-99.9%])</span> <div style='text-align: right'>321,024</div>  | <div style='text-align: right'>559</div>  | <div style='text-align: right'>68</div>  |  | <span style="color: green">(-1,048,064 [-100.0%])</span> <div style='text-align: right'>512</div>  |
| app_proof | VariableRangeCheckerAir | 0 | <div style='text-align: right'>2,359,296</div>  | <div style='text-align: right'>1</div>  | <div style='text-align: right'>8</div>  | <div style='text-align: right'>2</div>  | <div style='text-align: right'>262,144</div>  |

| segment | trace_gen_time_ms |
| --- | --- |
| 0 | <span style="color: red">(+2.0 [+0.8%])</span> <div style='text-align: right'>266.0</div>  |

</details>



<details>
<summary>Flamegraphs</summary>

[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5f164be46e34b99494a820e7eaf450eb416e98d6/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5f164be46e34b99494a820e7eaf450eb416e98d6/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5f164be46e34b99494a820e7eaf450eb416e98d6/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5f164be46e34b99494a820e7eaf450eb416e98d6/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.air_name.cells_used.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5f164be46e34b99494a820e7eaf450eb416e98d6/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.reverse.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5f164be46e34b99494a820e7eaf450eb416e98d6/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.reverse.svg)
[![](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5f164be46e34b99494a820e7eaf450eb416e98d6/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.svg)](https://axiom-public-data-sandbox-us-east-1.s3.us-east-1.amazonaws.com/benchmark/github/flamegraphs/5f164be46e34b99494a820e7eaf450eb416e98d6/fibonacci-2-2-1048476-64cpu-linux-arm64-mimalloc-app_proof.dsl_ir.opcode.frequency.svg)

</details>

Commit: https://github.com/axiom-crypto/afs-prototype/commit/5f164be46e34b99494a820e7eaf450eb416e98d6

Max Segment Length: 1048476

Instance Type: 64cpu-linux-arm64

Memory Allocator: mimalloc

[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/12271084287)
