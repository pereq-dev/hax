.PHONY: default lean clean

default: lean

lean:
	cargo hax extract lean-tutorial
	(cd proofs/lean-tutorial/lean && \
		elan default v4.35.0 && \
		lake exe cache get && \
		lake build)

clean:
	-rm -rf proofs/lean-tutorial/lean/LeanTutorial/Extraction
	-rm -rf proofs/lean-tutorial/lean/.lake
