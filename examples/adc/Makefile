.PHONY: lean clean
lean:
	cargo hax extract adc
	(cd proofs/adc/lean && \
		elan default v4.35.0 && \
		lake exe cache get && \
		lake build)

clean:
	-rm -rf proofs/adc/lean/Adc/Extraction
	-rm -rf proofs/adc/lean/.lake
