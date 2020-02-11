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
mod adobe_rgb_1998;

use crate::colorimetry::illuminants::Illuminant;
use cgmath::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ColorSpace {
    AdobeRgb1998,
}

impl ColorSpace {
    pub fn illuminant(&self) -> Illuminant {
        match &self {
            ColorSpace::AdobeRgb1998 => self::adobe_rgb_1998::ILLUMINANT,
        }
    }

    pub fn whitepoint(&self) -> Illuminant {
        match &self {
            ColorSpace::AdobeRgb1998 => self::adobe_rgb_1998::ILLUMINANT,
        }
    }

    pub fn rgb_to_xyz_matrix(&self) -> Matrix3<f64> {
        match &self {
            ColorSpace::AdobeRgb1998 => self::adobe_rgb_1998::RGB_TO_XYZ_MATRIX,
        }
    }

    pub fn xyz_to_rgb_matrix(&self) -> Matrix3<f64> {
        match &self {
            ColorSpace::AdobeRgb1998 => self::adobe_rgb_1998::XYZ_TO_RGB_MATRIX,
        }
    }
}

//cached!{
//    RGB_TO_RGB_MATRIX;
fn rgb_to_rgb_matrix(input: &ColorSpace, output: &ColorSpace) -> Matrix3<f64> {
    let input_matrix = input.rgb_to_xyz_matrix();
    let output_matrix = output.xyz_to_rgb_matrix();

    output_matrix * input_matrix
}
//}

impl std::fmt::Display for ColorSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ColorSpace::AdobeRgb1998 => write!(f, "Adobe RGB 1998"),
        }
    }
}
