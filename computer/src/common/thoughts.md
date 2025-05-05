<!-- To sum it up again, PCI-e is a class of connector used in just about every computer out there. M.2 is a type of connector, too, but it’s also a form factor characterized by small, slim components that fit in space-constrained computers, laptops, and tablets. M.2 ports are mostly used for M.2 NVMe SSDs, the fastest storage media on the market, while PCI-e ports are used for a much wider range of components.

The popularity of M.2 NVMe SSDs makes it difficult to compare them to 2.5-inch PCI-e SSDs (there really aren’t many out there anymore), but it’s safe to say that you’ll want to look in the M.2 category if you’re in the market for a blazing-fast SSD. -->


<!-- ///////////////////////////////////////// -->
<!-- 
    src
    ├── common
    │   ├── mod.rs                  # pub mod utils; pub mod constants; ...
    │   ├── utils
    │   │   ├── file.rs             # file IO utilities
    │   │   ├── string.rs           # string processing
    │   │   ├── time.rs             # time/date helpers
    │   │   └── mod.rs              # pub use self::file::*; ...
    │   ├── constants
    │   │   ├── hardware.rs         # e.g., CPU types, voltage ranges
    │   │   ├── ui.rs               # color codes, max buffer sizes
    │   │   └── mod.rs
    │   ├── traits
    │   │   ├── describable.rs      # traits for uniform hardware descriptions
    │   │   └── mod.rs
    │   ├── types
    │   │   ├── id.rs               # ID wrappers like DeviceId, MemoryId
    │   │   ├── mod.rs
    │   └── thoughts.md             # notes on design 
-->