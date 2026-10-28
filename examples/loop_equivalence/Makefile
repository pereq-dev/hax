.PHONY: lean clean
lean:
	cargo hax extract loop-equivalence
	(cd proofs/loop-equivalence/lean && \
   elan default v4.35.0 && \
   lake exe cache get && \
   lake build)

clean:
	-rm -rf proofs/loop-equivalence/lean/LoopEquivalence/Extraction
	-rm -rf proofs/loop-equivalence/lean/.lake
