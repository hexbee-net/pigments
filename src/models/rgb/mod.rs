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

//! # RGB (red, green, blue) color space

pub mod color_space;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,

    pub space: color_space::ColorSpace,
}

impl Rgb {
    fn to_rgb(&self, space: color_space::ColorSpace) -> &Rgb {
        // M = RGB_to_RGB_matrix(input_colorspace, output_colorspace, chromatic_adaptation_transform)
        // RGB = dot_vector(M, RGB)
        // return from_range_1(RGB)
        unimplemented!();
    }
}
