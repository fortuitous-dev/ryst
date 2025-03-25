.PHONY: clean tests unit_tests integration_tests install

tests: unit_tests integration_tests

unit_tests:
	cargo test --lib

integration_tests:
	$(MAKE) -C Podman tests

install:
	cargo install --path .

clean:
	$(MAKE) -C Podman clean || echo "Cleaned Podman folder"
	cargo clean
