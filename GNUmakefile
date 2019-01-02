ROOT_DIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
RFC_6890_TXT := $(ROOT_DIR)/src/ift-cli/rfc6890_entries.txt
RFC_6890_ENTRIES_RS := $(ROOT_DIR)/src/ift/rfc6890_entries.rs

.PHONY: pre-hook
pre-hook: lint fmt

.PHONY: lint
lint:
	cargo clippy

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: download
	curl -s https://tools.ietf.org/rfc/rfc6890.txt > $(RFC_6890_TXT)

.PHONY: gen-rfc-6890
gen-rfc-6890:
	# first create a dummy file
	echo "use crate::ip_rfc::Rfc6890;" > $(RFC_6890_ENTRIES_RS)
	echo "pub fn entries() -> Rfc6890 { Rfc6890 { entries: vec![] }}" >> $(RFC_6890_ENTRIES_RS)

	# then write the real one to a tmp loc
	echo "use crate::ip_rfc::Rfc6890;" > $(RFC_6890_ENTRIES_RS).tmp
	echo "use crate::ip_rfc::Rfc6890Entry;" >> $(RFC_6890_ENTRIES_RS).tmp
	echo "pub fn entries() -> Rfc6890 { Rfc6890 { entries: vec![" >> $(RFC_6890_ENTRIES_RS).tmp
	cargo run --bin ift-cli -- rfc 6890 >> $(RFC_6890_ENTRIES_RS).tmp
	echo "]}}" >> $(RFC_6890_ENTRIES_RS).tmp

	# then move it into the correct loc
	mv $(RFC_6890_ENTRIES_RS).tmp $(RFC_6890_ENTRIES_RS)

.PHONY: gen
gen: download gen-rfc-6890 fmt

.PHONY: test
test:
	cargo run --bin ift-cli -- eval 'GetInterfaceIP "lo0" | FilterIPv4'
	cargo run --bin ift-cli -- eval 'GetAllInterfaces | FilterForwardable'
	cargo run --bin ift-cli -- eval 'GetAllInterfaces | FilterGlobal'
