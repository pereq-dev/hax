.PHONY: lean clean
lean:
	cargo hax extract sha3
	(cd proofs/sha3/lean && \
		elan default v4.35.0 && \
		lake exe cache get && \
		lake build)

clean:
	-rm -rf proofs/sha3/lean/Sha3/Extraction
	-rm -rf proofs/sha3/lean/.lake
