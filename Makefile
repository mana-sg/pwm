BINARY_NAME = pwm
INSTALL_DIR = /usr/local/bin

build:
	cargo build --release -q

install: build
	@echo "Installing $(BINARY_NAME) to $(INSTALL_DIR)..."
	@sudo cp target/release/$(BINARY_NAME) $(INSTALL_DIR)
	@echo "Installation complete!"

uninstall:
	@echo "Removing $(BINARY_NAME) from $(INSTALL_DIR)..."
	@sudo rm -f $(INSTALL_DIR)/$(BINARY_NAME)
	@echo "Uninstallation complete!"

clean:
	@echo "Cleaning up..."
	@cargo clean
	@echo "Cleanup complete!"
