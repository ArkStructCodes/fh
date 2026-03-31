use rkyv::{Archive, Deserialize};

#[derive(Archive, Deserialize, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Archive, Deserialize, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub fn distance(&self, rhs: Self) -> f32 {
        let dx = rhs.x - self.x;
        let dy = rhs.y - self.y;
        let dz = rhs.z - self.z;
        return f32::sqrt(dx * dx + dy * dy + dz * dz);
    }

    pub fn within(&self, rhs: Self, radius: f32) -> bool {
        return f32::abs(self.distance(rhs)) - radius <= 0.0;
    }
}

#[derive(Archive, Deserialize, Debug)]
pub struct WheelParams {
    pub front_left: f32,
    pub front_right: f32,
    pub rear_left: f32,
    pub rear_right: f32,
}

#[derive(Archive, Deserialize, Debug)]
pub struct IWheelParams {
    pub front_left: i32,
    pub front_right: i32,
    pub rear_left: i32,
    pub rear_right: i32,
}

#[derive(Archive, Deserialize, Debug)]
pub struct Packet {
    // sled
    pub is_race_on: f32,
    pub timestamp_ms: u32,
    pub engine_max_rpm: f32,
    pub engine_idle_rpm: f32,
    pub current_engine_rpm: f32,
    pub accleration: Vector,
    pub velocity: Vector,
    pub angular_velocity: Vector,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub normalized_suspension_travel: WheelParams,
    pub tire_slip_ratio: WheelParams,
    pub wheel_rotation_speed: WheelParams,
    pub wheel_on_rumble_strip: IWheelParams,
    pub wheel_in_puddle_depth: WheelParams,
    pub surface_rumble: WheelParams,
    pub tire_slip_angle: WheelParams,
    pub tire_combined_slip: WheelParams,
    pub suspension_travel_meters: WheelParams,
    pub car_ordinal: i32,
    pub car_class: i32,
    pub car_performance_index: i32,
    pub drivetrain_type: i32,
    pub num_cylinders: i32,

    // horizon
    pub car_type: i32,
    unknown: [u8; 8],

    // dash
    pub position: Position,
    pub speed: f32,
    pub power: f32,
    pub torque: f32,
    pub tire_temp: WheelParams,
    pub boost: f32,
    pub fuel: f32,
    pub distance_traveled: f32,
    pub best_lap: f32,
    pub last_lap: f32,
    pub current_lap: f32,
    pub current_race_time: f32,
    pub lap_number: u16,
    pub race_position: u8,
    pub accel: u8,
    pub brake: u8,
    pub clutch: u8,
    pub handbrake: u8,
    pub gear: u8,
    pub steer: i8,
    pub normalized_driving_line: i8,
    pub normalized_ai_brake_difference: i8,
}

impl TryFrom<&[u8; size_of::<Packet>()]> for Packet {
    type Error = rkyv::rancor::Error;

    fn try_from(bytes: &[u8; size_of::<Packet>()]) -> Result<Self, Self::Error> {
        let archived = rkyv::access::<ArchivedPacket, _>(bytes)?;
        rkyv::deserialize(archived)
    }
}
