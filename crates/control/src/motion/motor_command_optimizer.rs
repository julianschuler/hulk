use color_eyre::Result;
use context_attribute::context;
use framework::{AdditionalOutput, MainOutput};
use serde::{Deserialize, Serialize};
use types::{
    joints::{Joints, JointsCommand},
    parameters::MotorCommandOptimizerParameters,
    sensor_data::SensorData,
};

#[derive(Deserialize, Serialize)]
pub struct MotorCommandOptimizer {
    position_offset: Joints<f32>,
    is_resetting: bool,
}

#[context]
pub struct CreationContext {}

#[context]
pub struct CycleContext {
    pub motor_commands: Input<JointsCommand<f32>, "motor_commands">,
    pub sensor_data: Input<SensorData, "sensor_data">,

    pub parameters:
        Parameter<MotorCommandOptimizerParameters, "motor_command_optimizer_parameters">,

    pub squared_position_offset_sum:
        AdditionalOutput<f32, "motor_position_optimization_offset_squared_sum">,
    pub position_offset: AdditionalOutput<Joints<f32>, "motor_position_optimization_offset">,
}

#[context]
#[derive(Default)]
pub struct MainOutputs {
    pub optimized_motor_commands: MainOutput<JointsCommand<f32>>,
}

impl MotorCommandOptimizer {
    pub fn new(_context: CreationContext) -> Result<Self> {
        Ok(Self {
            position_offset: Joints::default(),
            is_resetting: false,
        })
    }

    pub fn cycle(&mut self, mut context: CycleContext) -> Result<MainOutputs> {
        let currents = context.sensor_data.currents;
        let commands = *context.motor_commands;
        let params = context.parameters;

        let squared_position_offset_sum: f32 = self
            .position_offset
            .as_vec()
            .into_iter()
            .flatten()
            .map(|position| position.powf(2.0))
            .sum();

        if squared_position_offset_sum > params.offset_reset_threshold {
            self.is_resetting = true;
        }

        if self.is_resetting {
            self.position_offset = self.position_offset / params.offset_reset_speed;

            if squared_position_offset_sum
                < params.offset_reset_threshold / params.offset_reset_offset
            {
                self.is_resetting = false;
            }
        }

        let maximal_current = currents.as_vec().into_iter().flatten().fold(0.0, f32::max);

        let position_offset = params
            .optimization_direction
            .as_vec()
            .into_iter()
            .flatten()
            .zip(currents.as_vec().into_iter().flatten())
            .map(|(correction_direction, current)| {
                if current == maximal_current {
                    params.optimization_speed * correction_direction
                } else {
                    0.0
                }
            });

        let reset_threshold_reached = maximal_current >= params.optimization_current_threshold;
        if reset_threshold_reached && !self.is_resetting {
            self.position_offset = self.position_offset + Joints::from_iter(position_offset);
        }

        let mut optimized_stiffnesses = commands.stiffnesses;
        optimized_stiffnesses.left_arm.hand = 0.0;
        optimized_stiffnesses.right_arm.hand = 0.0;

        let optimized_commands = JointsCommand {
            positions: commands.positions + self.position_offset,
            stiffnesses: optimized_stiffnesses,
        };

        context
            .squared_position_offset_sum
            .fill_if_subscribed(|| squared_position_offset_sum);
        context
            .position_offset
            .fill_if_subscribed(|| self.position_offset);

        Ok(MainOutputs {
            optimized_motor_commands: optimized_commands.into(),
        })
    }
}
