use anyhow::{Context, Result};

use super::algorithm::Temperature;

pub trait CoolingSchedule {
    fn cooldown(&mut self);
    fn temperature(&self) -> Temperature;
    fn name(&self) -> String;
}

pub struct LinearCoolingSchedule {
    temperature: Temperature,
    iteration: u32,
    max_iteration: u32,
}

impl CoolingSchedule for LinearCoolingSchedule {
    fn cooldown(&mut self) {
        self.temperature = 1f32 - ((self.iteration + 1) as f32 / (self.max_iteration as f32));
        self.iteration += 1;
    }

    fn temperature(&self) -> Temperature {
        self.temperature
    }

    fn name(&self) -> String {
        format!("linear: iterations: {} ", self.max_iteration)
    }
}
impl LinearCoolingSchedule {
    pub fn new(max_iteration: u32) -> Self {
        LinearCoolingSchedule {
            temperature: 0f32,
            iteration: 0,
            max_iteration,
        }
    }
}

pub struct ExponentialCoolingSchedule {
    temperature: Temperature,
    iteration: u32,
    initial_temperature: Temperature,
    cooling_factor: f32,
}

impl ExponentialCoolingSchedule {
    pub fn new(cooling_factor: f32, initial_temperature: Temperature) -> Self {
        ExponentialCoolingSchedule {
            temperature: initial_temperature,
            iteration: 0,
            cooling_factor,
            initial_temperature,
        }
    }
}

impl CoolingSchedule for ExponentialCoolingSchedule {
    fn cooldown(&mut self) {
        self.temperature *= self.cooling_factor;
        self.iteration += 1;
    }

    fn temperature(&self) -> Temperature {
        self.temperature
    }

    fn name(&self) -> String {
        format!(
            "exponential: initial_temp: {}, cooling_factor: {} ",
            self.initial_temperature, self.cooling_factor
        )
    }
}

#[derive(Default)]
pub struct ExponentialCoolingScheduleBuilder {
    initial_temperature: Option<Temperature>,
    cooling_factor: Option<f32>,
}

impl ExponentialCoolingScheduleBuilder {
    pub fn new() -> Self {
        ExponentialCoolingScheduleBuilder::default()
    }

    pub fn initial_temperature(mut self, initial_temperature: Temperature) -> Self {
        self.initial_temperature = Some(initial_temperature);
        self
    }

    pub fn cooling_factor(mut self, cooling_factor: f32) -> Self {
        self.cooling_factor = Some(cooling_factor);
        self
    }

    pub fn build(self) -> Result<ExponentialCoolingSchedule> {
        Ok(ExponentialCoolingSchedule {
            temperature: self
                .initial_temperature
                .context("No initial temperature was set")?,
            iteration: 0,
            cooling_factor: self.cooling_factor.context("No cooling factor")?,
            initial_temperature: self
                .initial_temperature
                .context("No initial temperature was set")?,
        })
    }
}
