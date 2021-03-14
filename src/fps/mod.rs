use sdl2::{Sdl, TimerSubsystem};
use std::time::{Duration, SystemTime};
use std::thread::sleep;

const NUM_SAMPLES: u32 = 10;

pub struct FpsCalculator {
    fps: f32,
    frame_time: f32,
    current_frame: u32,
    previous_ticks: u32,
    frame_times: [f32; NUM_SAMPLES as usize],
}

impl FpsCalculator {
    pub fn new() -> FpsCalculator {
        FpsCalculator {
            fps: 0.0,
            frame_time: 0.0,
            current_frame: 0,
            previous_ticks: 0,
            frame_times: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn start(&mut self, sdl_time_subsystem: &mut TimerSubsystem) {
        self.previous_ticks = sdl_time_subsystem.ticks();
    }

    pub fn calculate_fps(&mut self, sdl_time_subsystem: &mut TimerSubsystem) -> f32 {
        let mut currentTicks: u32 = sdl_time_subsystem.ticks();

        self.frame_time = (currentTicks - self.previous_ticks) as f32;
        let index = (self.current_frame % NUM_SAMPLES) as usize;
        self.frame_times[index] = self.frame_time;


        self.previous_ticks = currentTicks;

        let mut count = 0;

        if (self.current_frame < NUM_SAMPLES) {
            count = self.current_frame;
        } else {
            count = NUM_SAMPLES;
        }

        let mut frame_time_average: f32 = self.frame_times.iter().sum::<f32>() / count as f32;




        if (frame_time_average > 0.0) {
            self.fps = 1000.0 / frame_time_average;
        } else {
            self.fps = 60.0; //this will never happen except probably on first frame
        }

        self.current_frame += 1; //todo overflow at some point?

        self.fps
    }
}
