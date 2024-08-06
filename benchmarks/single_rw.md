## Benchmark for ReadWrite
| Total Cells | Total Prove (ms) | Main Trace Gen (ms) | Perm Trace Gen (ms) | Calc Quotient Values (ms) | Rest of Prove (ms) |
|-----------------------------|-----------------------|--------------------------|--------------------------|-----------------|----------------|
| 38_764_544 | 1490.00 | 287.00 | 20.30 | 171.00 | 1011.70 |

### AIR metrics
| Name | Rows | Cells | Prep Cols | Main Cols | Perm Cols |
|------|------|-------|-----------|-----------|-----------|
| PageReadAir          | 32_768     | 1_343_488   | 0     | [33] | [8] |
| IndexedPageWriteAir  | 32_768     | 5_603_328   | 0     | [33, 98] | [40] |
| PageOfflineChecker   | 131_072    | 29_753_344  | 0     | [179] | [48] |
| RangeCheckerGateAir  | 65_536     | 655_360     | 0     | [2] | [8] |
| ExecutionAir         | 32_768     | 1_409_024   | 0     | [35] | [8] |

Commit: https://github.com/axiom-crypto/afs-prototype/commit/d3ea42b4dade3b458e2a87807a144e1e53d59f3f
AWS Instance Type: [r7g.8xlarge](https://instances.vantage.sh/aws/ec2/r7g.8xlarge)
[Benchmark Workflow](https://github.com/axiom-crypto/afs-prototype/actions/runs/10275495808)
