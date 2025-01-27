#!/bin/bash

# Base directory
BASE_DIR="src/hardware"

# Create directories and files
create_dir_and_file() {
    mkdir -p "$(dirname "$1")"
    touch "$1"
    echo "Created: $1"
}

# Memory directory
MEMORY_FILES=(
    "memory/mod.rs"
    # Cache hierarchy
    "memory/cache/mod.rs"
    "memory/cache/l1_cache.rs"
    "memory/cache/l2_cache.rs"
    "memory/cache/l3_cache.rs"
    "memory/cache/coherency.rs"
    "memory/cache/replacement.rs"
    "memory/cache/prefetch.rs"
    "memory/cache/stats.rs"
    # DRAM components
    "memory/dram/mod.rs"
    "memory/dram/bank.rs"
    "memory/dram/rank.rs"
    "memory/dram/timing.rs"
    "memory/dram/refresh.rs"
    "memory/dram/voltage.rs"
    "memory/dram/temperature.rs"
    "memory/dram/ecc.rs"
    # Memory Controller
    "memory/controller/mod.rs"
    "memory/controller/scheduler.rs"
    "memory/controller/power.rs"
    "memory/controller/timing.rs"
    "memory/controller/queue.rs"
    # Memory Management Unit
    "memory/mmu/mod.rs"
    "memory/mmu/paging.rs"
    "memory/mmu/tlb.rs"
    "memory/mmu/protection.rs"
    "memory/mmu/virtual_memory.rs"
    # Power Management
    "memory/power/mod.rs"
    "memory/power/voltage.rs"
    "memory/power/frequency.rs"
    "memory/power/thermal.rs"
    # Training and Calibration
    "memory/training/mod.rs"
    "memory/training/write_leveling.rs"
    "memory/training/read_training.rs"
    "memory/training/gate_training.rs"
    "memory/training/voltage_training.rs"
    # Error Management
    "memory/error/mod.rs"
    "memory/error/ecc.rs"
    "memory/error/scrubbing.rs"
    "memory/error/logging.rs"
    # Bus Interface
    "memory/bus/mod.rs"
    "memory/bus/data_bus.rs"
    "memory/bus/address_bus.rs"
    "memory/bus/command_bus.rs"
    # Statistics and Monitoring
    "memory/stats/mod.rs"
    "memory/stats/performance.rs"
    "memory/stats/power.rs"
    "memory/stats/temperature.rs"
    "memory/stats/errors.rs"
)

# CPU directory
CPU_FILES=(
    "cpu/mod.rs"
    "cpu/alu.rs"
    "cpu/registers.rs"
    "cpu/pipeline.rs"
    "cpu/cache_controller.rs"
    "cpu/instruction_decoder.rs"
    "cpu/branch_predictor.rs"
    "cpu/execution_unit.rs"
)

# GPU directory
GPU_FILES=(
    "gpu/mod.rs"
    "gpu/compute/mod.rs"
    "gpu/compute/shader_core.rs"
    "gpu/compute/tensor_core.rs"
    "gpu/compute/ray_core.rs"
    "gpu/memory/mod.rs"
    "gpu/memory/vram.rs"
    "gpu/memory/cache.rs"
    "gpu/memory/controller.rs"
    "gpu/display/mod.rs"
    "gpu/display/rasterizer.rs"
    "gpu/display/framebuffer.rs"
    "gpu/display/output.rs"
    "gpu/scheduler/mod.rs"
    "gpu/scheduler/workload.rs"
    "gpu/scheduler/dispatcher.rs"
)

# Storage directory
STORAGE_FILES=(
    "storage/mod.rs"
    "storage/disk/mod.rs"
    "storage/disk/platter.rs"
    "storage/disk/head.rs"
    "storage/disk/cache.rs"
    "storage/disk/controller.rs"
    "storage/ssd/mod.rs"
    "storage/ssd/nand.rs"
    "storage/ssd/controller.rs"
    "storage/ssd/wear_leveling.rs"
    "storage/ssd/garbage_collection.rs"
    "storage/nvme/mod.rs"
    "storage/nvme/protocol.rs"
    "storage/nvme/queue.rs"
    "storage/nvme/controller.rs"
    "storage/filesystem/mod.rs"
    "storage/filesystem/fat.rs"
    "storage/filesystem/ntfs.rs"
    "storage/filesystem/ext4.rs"
)

# Bus directory
BUS_FILES=(
    "bus/mod.rs"
    "bus/system_bus.rs"
    "bus/memory_bus.rs"
    "bus/pci_bus.rs"
    "bus/arbitration.rs"
)

# IO directory
IO_FILES=(
    "io/mod.rs"
    "io/controllers/mod.rs"
    "io/controllers/usb.rs"
    "io/controllers/sata.rs"
    "io/controllers/network.rs"
    "io/devices/mod.rs"
    "io/devices/storage.rs"
    "io/devices/display.rs"
    "io/devices/input.rs"
)

# Create all directories and files
echo "Creating hardware directory structure..."

# Create memory files first (since they're most important for your current focus)
for file in "${MEMORY_FILES[@]}"; do
    create_dir_and_file "$BASE_DIR/$file"
done

# Create other hardware component files
for file in "${CPU_FILES[@]}" "${GPU_FILES[@]}" "${STORAGE_FILES[@]}" "${BUS_FILES[@]}" "${IO_FILES[@]}"; do
    create_dir_and_file "$BASE_DIR/$file"
done

echo "Directory structure created successfully!"

# Optional: Initialize mod.rs files with basic exports
for dir in $(find "$BASE_DIR" -type d); do
    if [ -f "$dir/mod.rs" ]; then
        echo "// Export all modules in $(basename $dir)" > "$dir/mod.rs"
        for file in "$dir"/*.rs; do
            if [ "$(basename $file)" != "mod.rs" ]; then
                module_name=$(basename $file .rs)
                echo "pub mod $module_name;" >> "$dir/mod.rs"
            fi
        done
    fi
done 