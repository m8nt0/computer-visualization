<!-- laptop/
└── common/
    ├── mod.rs
    ├── config/
    │   ├── mod.rs
    │   ├── thermal_limits.rs     # Laptop-specific thermal thresholds
    │   ├── power_profiles.rs     # Battery/performance profiles
    │   └── sleep_settings.rs     # Suspend/sleep config
    ├── utils/
    │   ├── mod.rs
    │   ├── battery.rs            # Battery percent/health logic
    │   ├── fan.rs                # Fan curve helpers
    │   ├── throttling.rs         # CPU/GPU throttling logic
    │   └── power.rs              # Power draw and efficiency utils
    ├── traits/
    │   ├── mod.rs
    │   ├── throttleable.rs       # Trait for throttling-capable components
    │   ├── monitorable.rs        # For telemetry-capable hardware
    │   └── chargeable.rs         # Trait for components that store charge
    ├── types/
    │   ├── mod.rs
    │   ├── battery_health.rs     # Structs/enums for battery health
    │   ├── thermal.rs            # Laptop-specific temp readings
    │   └── perf_state.rs         # Performance states (P-States, etc.)
    └── validation/
        ├── mod.rs
        ├── thermal.rs            # Validate temps are within safe range
        └── power.rs              # Ensure current/power limits aren't exceeded -->