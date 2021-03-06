use std::f64::consts::PI;

#[derive(Clone, Debug)]
pub struct MouseEuler {
    pub pitch: f64,
    pub yaw: f64,
}

pub struct MouseSensitivity {
    sensitivity: f64,
    pitch: f64,
    yaw: f64,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct MouseConfig {
    pub sensitivity: f64,
}

impl Default for MouseConfig {
    fn default() -> MouseConfig {
        MouseConfig {
            sensitivity: 5.0,
        }
    }
}

const HALF_PI: f64 = PI / 2.0;

impl Default for MouseEuler {
    fn default() -> MouseEuler {
        MouseEuler { pitch: 0.0, yaw: 0.0 }
    }
}

impl MouseEuler {
    pub fn update(&mut self, delta: (f64, f64), sensitivity: &MouseSensitivity) {
        let pitch_mult = sensitivity.sensitivity * sensitivity.pitch;
        let yaw_mult = sensitivity.sensitivity * sensitivity.yaw;

        self.pitch += delta.1 * pitch_mult;
        if self.pitch > HALF_PI {
            self.pitch = HALF_PI;
        } else if self.pitch < -HALF_PI {
            self.pitch = -HALF_PI;
        }

        self.yaw -= delta.0 * yaw_mult;
        if self.yaw > PI {
            self.yaw = -PI + self.yaw % PI;
        } else if self.yaw < -PI {
            self.yaw = PI + self.yaw % PI;
        }
    }
}

impl MouseSensitivity {
    pub fn new(sensitivity: f64) -> MouseSensitivity {
        MouseSensitivity {
            pitch: 0.0022f64,
            yaw: 0.0022f64,
            sensitivity,
        }
    }
}