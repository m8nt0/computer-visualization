use super::bus::Bus;

pub mod alu;
pub mod branch_predictor;
pub mod cache_controller;
pub mod execution_unit;
pub mod instruction_decoder;
pub mod pipeline;
pub mod registers;

pub struct CPU {
    registers: registers::RegisterFile,
    pipeline: pipeline::Pipeline,
    branch_predictor: branch_predictor::BranchPredictor,
    cache_controller: cache_controller::CacheController,
    execution_unit: execution_unit::ExecutionUnit,
    instruction_decoder: instruction_decoder::InstructionDecoder,
    
    // CPU state
    frequency: u32,      // Current clock frequency
    temperature: f32,    // Current temperature
    power_state: u8,     // Current power state (C0, C1, etc)
    bus: *mut Bus,       // System bus connection
}

impl CPU {
    pub fn new(bus: *mut Bus) -> Self {
        let registers = registers::RegisterFile::new();
        
        Self {
            registers,
            pipeline: pipeline::Pipeline::new(registers.clone()),
            branch_predictor: branch_predictor::BranchPredictor::new(1024),
            cache_controller: cache_controller::CacheController::new(bus),
            execution_unit: execution_unit::ExecutionUnit::new(&registers as *mut _),
            instruction_decoder: instruction_decoder::InstructionDecoder::new(),
            frequency: 3_000_000_000,
            temperature: 40.0,
            power_state: 0,
            bus,
        }
    }

    pub fn tick(&mut self) {
        // Update pipeline
        self.pipeline.tick();
        
        // Execute current instruction
        if let Some(instruction) = self.pipeline.get_current_instruction() {
            let decoded = self.instruction_decoder.decode(instruction);
            self.execution_unit.execute(decoded);
        }
        
        // Update execution unit
        self.execution_unit.tick();
        
        // Update temperature based on activity
        self.update_temperature();
    }

    fn update_temperature(&mut self) {
        // Simple temperature model based on CPU activity
        let activity_factor = if self.execution_unit.is_busy() { 1.0 } else { 0.1 };
        let ambient_temp = 25.0;
        let max_temp = 90.0;
        
        self.temperature = self.temperature * 0.99 + 
            (ambient_temp + (max_temp - ambient_temp) * activity_factor) * 0.01;
    }

    // Methods for visualization system
    pub fn get_frequency(&self) -> u32 {
        self.frequency
    }

    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }

    pub fn get_power_state(&self) -> u8 {
        self.power_state
    }

    pub fn get_pipeline_state(&self) -> &pipeline::Pipeline {
        &self.pipeline
    }

    pub fn get_branch_predictor(&self) -> &branch_predictor::BranchPredictor {
        &self.branch_predictor
    }

    pub fn get_cache_controller(&self) -> &cache_controller::CacheController {
        &self.cache_controller
    }

    pub fn get_execution_unit(&self) -> &execution_unit::ExecutionUnit {
        &self.execution_unit
    }
}

