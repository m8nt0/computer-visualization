# 🖥️ Computer Architecture Visualization

A modern, Rust-based visualization system for computer architecture components and their interactions. This project provides an educational and interactive way to understand how computers work at a hardware level.

## 📌 Topics
`rust` `computer-architecture` `visualization` `educational` `computer-science` `hardware` `cpu` `gpu` `memory-management` `system-design` `computer-engineering` `simulation` `rust-lang` `low-level` `performance`

## 🌟 Features

- **CPU Visualization**
  - Pipeline stages
  - ALU operations
  - Cache hierarchy
- **Memory System**
  - DRAM controller
  - Cache hierarchy
  - Memory management
- **Storage Components**
  - Disk operations
  - I/O handling
- **GPU Architecture**
  - Compute units
  - Graphics pipeline
- **System Bus**
  - Data transfer
  - Communication protocols

## 🚀 Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)

### Installation

```bash
# Clone the repository
git clone https://github.com/m8nt0/computer-visualization.git

# Change into the project directory
cd computer-visualization

# Build the project
cargo build

# Run the project
cargo run
```

## 🏗️ Project Structure

```
computer/
├── src/              # Core implementation
└── visualization/    # Visualization components
    ├── hardware/
    │   ├── cpu/     # CPU components
    │   ├── gpu/     # GPU architecture
    │   ├── memory/  # Memory hierarchy
    │   ├── storage/ # Storage systems
    │   └── bus/     # System bus
    └── computer/    # High-level computer system
```

## 🛠️ Technology Stack

- **Language**: Rust
- **Build Tool**: Cargo
- **Architecture**: Modular component-based design agnostic of external frameworks/libraries

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ✨ Future Outlooks

Computer
│
├── Stationary Comp.
│   ├── Desktop
│   ├── Server
│   └── Supercomputer
│
├── Mobile Comp.
│   ├── Smartphone
│   ├── Tablet
|   ├── Laptops
│   ├── Smartwatch
│   └── eReader (like Kindle)
│
├── Embedded / IoT Computers
│   ├── Smart Appliances
│   ├── Smart TVs
│   ├── Smart Home Devices
│   ├── Wearables
│   ├── Smart Sensors
│   ├── Cars (ECUs, ADAS, Infotainment)
│   └── Planes (Avionics, FMS, Engine Controls)
|
├── Specialized Computers
│   ├── Gaming Consoles
│   ├── Point-of-Sale Terminals
│   ├── Medical Devices
│   └── Industrial Controllers

<!-- ////////////////////// -->
## computing

                            COMPUTER SYSTEM
                                 │
                ┌────────────────┴────────────────┐
             HARDWARE                             SOFTWARE
         (The physical machine)             (The instructions, behavior)
                │                                     │
     ┌──────────┴──────────┐          ┌──────────────┴───────────────┐
  PROCESSING          SUPPORT     SYSTEM SOFTWARE             APPLICATION SOFTWARE
(CPU, GPU, RAM)   (Power, Cooling) (OS, Drivers)           (Apps, Games, Browsers)


- somebody makes the device hardware. boom
- the people who made it boot it thourgh the porst like usb, an os.
- then it goes to the storage.
- the user buys the computing devices, turns it on.
- the firmware checks hardware, and copies OS first from storage to ram, and to cpu.
- the user has to sign in.
- then it goes back to the storage to get the kernel.
- then goes back to the storage after successful sign in, and get the apps from storage and come in.
- then when does drivers come up?
- when you make a change, it updates both the ram and storage.
- does the os control network, screen looks, and keyboards?
- How many stages and levels?



<!-- ///////////////////////////////////// -->

Computer
├── Hardware
│   ├── Processing
│   │   ├── CPU
│   │   │   ├── Control Unit (CU)
│   │   │   └── Arithmetic Logic Unit (ALU)
│   │   └── GPU
│   │       ├── Rendering Engine
│   │       └── Video Memory (VRAM)
│   └── Peripherals
│       ├── Memory
│       │   ├── Volatile
│       │   │   ├── DRAM
│       │   │   └── SRAM
│       │   └── Non-Volatile
│       │       ├── SSD
│       │       └── HDD
│       └── I/O Devices
│           ├── Input
│           │   ├── Manual (Keyboard, Mouse)
│           │   └── Sensor-Based (Camera, Mic)
│           └── Output
│               ├── Visual (Monitor)
│               └── Audio (Speakers)
└── Software
    ├── System Software
    │   ├── Kernel
    │   │   ├── Monolithic Kernel
    │   │   └── Microkernel
    │   └── Drivers
    │       ├── Device Drivers
    │       └── Virtual Drivers
    └── Application Software
        ├── User Applications
        │   ├── GUI-Based Apps
        │   └── CLI-Based Apps
        └── Services
            ├── Background Services (daemons)
            └── Scheduled Tasks (cron jobs, schedulers)