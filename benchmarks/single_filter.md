## Benchmark for Predicate
| Total Cells | Total Prove (ms) | Main Trace Gen (ms) | Perm Trace Gen (ms) | Calc Quotient Values (ms) | Rest of Prove (ms) |
|-----------------------------|-----------------------|--------------------------|--------------------------|-----------------|----------------|
| 19_726_336 | 722.00 | 214.00 | 8.81 | 30.10 | 469.09 |

### AIR metrics
| Name | Rows | Cells | Prep Cols | Main Cols | Perm Cols |
|------|------|-------|-----------|-----------|-----------|
| PageIndexScanInputAir | 32_768     | 10_354_688  | 0     | [97, 147] | [72] |
| PageIndexScanOutputAir | 32_768     | 8_716_288   | 0     | [97, 97] | [72] |
| RangeCheckerGateAir  | 65_536     | 655_360     | 0     | [2] | [8] |

Commit: https://github.com/axiom-crypto/afs-prototype/commit/0b6a6f94156780bf7b6d1c6339613a9d221df2ea
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/10424081325)
