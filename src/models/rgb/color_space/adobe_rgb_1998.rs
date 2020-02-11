// Copyright © 2020 Xavier Basty <xavier@hexbee.net>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::colorimetry::illuminants::Illuminant;

use cgmath::*;

pub const ILLUMINANT: Illuminant = Illuminant::D65;
pub const WHITEPOINT: Illuminant = Illuminant::D65;

pub const RGB_TO_XYZ_MATRIX: Matrix3<f64> = Matrix3 {
    x: Vector3{x:0.57667, y:0.29734, z:0.02703},
    y: Vector3{x:0.18556, y:0.62736, z:0.07069},
    z: Vector3{x:0.18823, y:0.07529, z:0.99134}
};

pub const XYZ_TO_RGB_MATRIX: Matrix3<f64> = Matrix3 {
    x: Vector3{x: 2.04159, y:-0.96924, z: 0.01344},
    y: Vector3{x:-0.56501, y: 1.87597, z:-0.11836},
    z: Vector3{x:-0.34473, y: 0.04156, z: 1.01517}
};
