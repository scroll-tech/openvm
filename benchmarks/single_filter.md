## Benchmark for Predicate
| Total Cells | Total Prove (ms) | Main Trace Gen (ms) | Perm Trace Gen (ms) | Calc Quotient Values (ms) | Rest of Prove (ms) |
|-----------------------------|-----------------------|--------------------------|--------------------------|-----------------|----------------|
| 18_677_760 | 669.00 | 179.00 | 8.25 | 28.30 | 453.45 |

### AIR metrics
| Name | Rows | Cells | Prep Cols | Main Cols | Perm Cols |
|------|------|-------|-----------|-----------|-----------|
| PageIndexScanInputAir | 32_768     | 9_830_400   | 0     | [97, 131] | [72] |
| PageIndexScanOutputAir | 32_768     | 8_192_000   | 0     | [97, 81] | [72] |
| RangeCheckerGateAir  | 65_536     | 655_360     | 0     | [2] | [8] |

Commit: https://github.com/axiom-crypto/afs-prototype/commit/3ab9399da68817d146da71d6f8116f77cefd798a
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/10709315097)
