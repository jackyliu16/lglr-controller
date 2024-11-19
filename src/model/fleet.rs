#![allow(unused_variables)]
use super::position::Position;

#[derive(Debug, Clone)]
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
}
