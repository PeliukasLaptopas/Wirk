use legion::*;

pub struct Ecs {
    pub schedule: Schedule,
    pub resources: Resources,
    pub world: World
}

impl Ecs {
    pub fn run(&mut self) {
        self.schedule.execute(&mut self.world, &mut self.resources);
    }
}