


fuzz-replication:
	cargo fuzz run fuzz_s3_replication -- -runs=15000
	cargo fuzz coverage fuzz_s3_replication
	cargo cov -- export \
		target/x86_64-apple-darwin/coverage/x86_64-apple-darwin/release/fuzz_s3_replication \
		-instr-profile=fuzz/coverage/fuzz_s3_replication/coverage.profdata --format=lcov >lcov.info


