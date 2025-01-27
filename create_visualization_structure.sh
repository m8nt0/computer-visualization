#!/bin/bash

# Base directory for visualization
VIS_DIR="computer/visualization"

# Function to create directory and file
create_dir_and_file() {
    mkdir -p "$(dirname "$1")"
    touch "$1"
    echo "Created: $1"
}

# Computer view (entry point) files
COMPUTER_FILES=(
    "computer/mod.rs"
    "computer/laptop.rs"
    "computer/screen.rs"
    "computer/keyboard.rs"
    "computer/ports.rs"
    "computer/power.rs"
)

# Hardware visualization files
HARDWARE_FILES=(
    "hardware/mod.rs"
    # CPU visualization
    "hardware/cpu/mod.rs"
    "hardware/cpu/core.rs"
    "hardware/cpu/cache.rs"
    "hardware/cpu/pipeline.rs"
    # Memory visualization
    "hardware/memory/mod.rs"
    "hardware/memory/cache.rs"
    "hardware/memory/dram.rs"
    "hardware/memory/bus.rs"
    # GPU visualization
    "hardware/gpu/mod.rs"
    "hardware/gpu/cores.rs"
    "hardware/gpu/memory.rs"
    # Common components
    "hardware/common/mod.rs"
    "hardware/common/data_flow.rs"
    "hardware/common/temperature.rs"
)

# Software visualization files
SOFTWARE_FILES=(
    "software/mod.rs"
    # OS visualization
    "software/os/mod.rs"
    "software/os/desktop.rs"
    "software/os/windows.rs"
    "software/os/taskbar.rs"
    # Process visualization
    "software/processes/mod.rs"
    "software/processes/task.rs"
    "software/processes/memory.rs"
    # Filesystem visualization
    "software/filesystem/mod.rs"
    "software/filesystem/explorer.rs"
)

echo "Creating visualization directory structure..."

# Create all directories and files
for file in "${COMPUTER_FILES[@]}" "${HARDWARE_FILES[@]}" "${SOFTWARE_FILES[@]}"; do
    create_dir_and_file "$VIS_DIR/$file"
done

# Initialize main mod.rs
cat > "$VIS_DIR/mod.rs" << 'EOL'
//! Visualization system for computer hardware simulation
//! This module provides visual representations of computer components
//! for educational purposes

mod computer;
mod hardware;
mod software;

pub use computer::ComputerView;
pub use hardware::HardwareView;
pub use software::SoftwareView;

/// Main visualization system that coordinates all views
pub struct VisualizationSystem {
    current_view: ViewMode,
    computer_view: ComputerView,
    hardware_view: HardwareView,
    software_view: SoftwareView,
}
EOL

echo "Directory structure created successfully!" 