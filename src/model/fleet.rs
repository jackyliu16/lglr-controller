#![allow(unused_variables)]
use super::position::Position;

#[derive(Debug)]
pub struct Fleet {
    belonger: String,                 // 舰队归属
    name: String,                     // 舰队名称
    position: Position,               // 当前所在位置
    conventional_engine_speed: usize, // 常规引擎航速
    curvature_engine_speed: usize,    // 曲率引擎航速
    info: String,
}

impl Fleet {
    pub fn new(
        belonger: String,
        name: String,
        position: Position,
        conventional_engine_speed: usize,
        curvature_engine_speed: usize,
        info: String,
    ) -> Self {
        Self {
            belonger,
            name,
            position,
            conventional_engine_speed,
            curvature_engine_speed,
            info,
        }
    }

    pub(crate) fn ref_array(&self) -> [String; 6] {
        [
            self.belonger.clone(),
            self.name.clone(),
            self.position.to_string(),
            self.conventional_engine_speed.to_string(),
            self.curvature_engine_speed.to_string(),
            self.info.clone(),
        ]
    }

    pub fn belonger(&self) -> &str {
        &self.belonger
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn conventional_engine_speed(&self) -> usize {
        self.conventional_engine_speed
    }

    pub fn curvature_engine_speed(&self) -> usize {
        self.curvature_engine_speed
    }

    pub fn info(&self) -> &str {
        &self.info
    }
}
