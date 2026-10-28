.PHONY: default fstar rust lean clean

default: fstar rust lean
	make -C proofs/barrett/fstar/extraction

fstar:
	cargo hax extract barrett-fstar

# The rust backend has no proof-scenario support.
rust:
	cargo hax into rust
	cd proofs/rust/extraction && cargo build

lean:
	cargo hax extract barrett
	(cd proofs/barrett/lean && \
		elan default v4.35.0 && \
		lake exe cache get && \
		lake build)

clean:
	rm -f proofs/barrett/fstar/extraction/.depend
	rm -f proofs/barrett/fstar/extraction/*.fst
	rm -f proofs/rust/extraction/*.rs
	-rm -rf proofs/barrett/lean/Barrett/Extraction
	-rm -rf proofs/barrett/lean/.lake
