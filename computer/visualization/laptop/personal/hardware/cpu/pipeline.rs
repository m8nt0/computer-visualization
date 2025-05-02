use super::common::{Point, Size, Color, Rect};
use crate::hardware::cpu::pipeline::{Pipeline, PipelineStage, Instruction};

pub struct PipelineVisualizer {
    position: Point,
    size: Size,
    stages: Vec<StageView>,
    data_flows: Vec<DataFlow>,
}

struct StageView {
    stage_type: PipelineStage,
    region: Rect,
    instruction: Option<Instruction>,
    stalled: bool,
    hazard: Option<HazardType>,
}

impl PipelineVisualizer {
    pub fn new(position: Point, size: Size) -> Self {
        let stage_width = size.width / 5.0;
        let stages = vec![
            StageView::new(PipelineStage::Fetch, position),
            StageView::new(PipelineStage::Decode, position + Point::new(stage_width, 0.0)),
            StageView::new(PipelineStage::Execute, position + Point::new(stage_width * 2.0, 0.0)),
            StageView::new(PipelineStage::Memory, position + Point::new(stage_width * 3.0, 0.0)),
            StageView::new(PipelineStage::Writeback, position + Point::new(stage_width * 4.0, 0.0)),
        ];

        Self {
            position,
            size,
            stages,
            data_flows: Vec::new(),
        }
    }

    pub fn update(&mut self, pipeline: &Pipeline) {
        // Update stage states
        for (i, stage) in self.stages.iter_mut().enumerate() {
            stage.instruction = pipeline.get_instruction_at_stage(i);
            stage.stalled = pipeline.is_stage_stalled(i);
            stage.hazard = pipeline.get_hazard_at_stage(i);
        }

        // Update data flows
        self.update_data_flows(pipeline);
    }

    pub fn render(&self, frame: &mut Frame) {
        // Draw pipeline structure
        self.draw_pipeline_structure(frame);
        
        // Draw stages
        for stage in &self.stages {
            stage.render(frame);
        }
        
        // Draw data flows
        for flow in &self.data_flows {
            flow.render(frame);
        }
        
        // Draw hazard indicators
        self.draw_hazards(frame);
        
        // Draw performance metrics
        self.draw_metrics(frame);
    }

    fn draw_hazards(&self, frame: &mut Frame) {
        for stage in &self.stages {
            if let Some(hazard) = &stage.hazard {
                let color = match hazard {
                    HazardType::Data => Color::RED,
                    HazardType::Control => Color::YELLOW,
                    HazardType::Structural => Color::ORANGE,
                };
                
                let hazard_rect = stage.region.shrink(4.0);
                frame.draw_rect_outline(hazard_rect, color, 2.0);
            }
        }
    }
}
