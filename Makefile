BINARY_NAME = pwm
BUILD_DIR = target/release

# Detect OS
ifeq ($(OS),Windows_NT)
    INSTALL_DIR = C:\Program Files\$(BINARY_NAME)
    COPY_CMD = copy /Y
    RM_CMD = del /F /Q
    MKDIR_CMD = if not exist "$(INSTALL_DIR)" mkdir "$(INSTALL_DIR)"
    RMDIR_CMD = rmdir /S /Q
else ifeq ($(shell uname),Linux)
    INSTALL_DIR = /usr/local/bin
    COPY_CMD = sudo cp
    RM_CMD = sudo rm -f
    MKDIR_CMD = sudo mkdir -p
    RMDIR_CMD = sudo rm -rf
else ifeq ($(shell uname),Darwin)
    INSTALL_DIR = /usr/local/bin
    COPY_CMD = sudo cp
    RM_CMD = sudo rm -f
    MKDIR_CMD = sudo mkdir -p
    RMDIR_CMD = sudo rm -rf
else
    @echo "Error: Unidentified OS detected!" >&2	
    $(error Unidentified OS, terminating...)
endif

build:
	cargo build --release -q

install: build
	@echo "Installing $(BINARY_NAME) to $(INSTALL_DIR)..."
	@$(MKDIR_CMD) "$(INSTALL_DIR)"
	@$(COPY_CMD) "$(BUILD_DIR)/$(BINARY_NAME)" "$(INSTALL_DIR)"
	@echo "Installation complete!"

uninstall:
	@echo "Removing $(BINARY_NAME) from $(INSTALL_DIR)..."
	@$(RM_CMD) "$(INSTALL_DIR)/$(BINARY_NAME)"
	@echo "Uninstallation complete!"

clean:
	@echo "Cleaning up..."
	@cargo clean
	@echo "Cleanup complete!"
