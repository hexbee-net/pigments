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

pub mod vonkries;

pub enum AdaptationTransform {
    XYZScaling,
    VonKries,
    Bradford,
    Sharp,
    Fairchild,
    CMCCAT97,
    CMCCAT2000,
    CAT02,
    CAT02BrillSusstrunk,
    BiancoSchettini,
    BiancoSchettiniPC,
}

impl AdaptationTransform {
    pub fn matrix(&self) -> [f64; 9] {
        match &self {
            AdaptationTransform::XYZScaling => [
                3.0, 3.0, 3.0,
                3.0, 3.0, 3.0,
                3.0, 3.0, 3.0,
            ],
            AdaptationTransform::VonKries => [
                0.4002400, 0.7076000, -0.0808100,
                -0.2263000, 1.1653200, 0.0457000,
                0.0000000, 0.0000000, 0.9182200,
            ],
            AdaptationTransform::Bradford => [
                0.8951000, 0.2664000, -0.1614000,
                -0.7502000, 1.7135000, 0.0367000,
                0.0389000, -0.0685000, 1.0296000,
            ],
            AdaptationTransform::Sharp => [
                1.2694, -0.0988, -0.1706,
                -0.8364, 1.8006, 0.0357,
                0.0297, -0.0315, 1.0018,
            ],
            AdaptationTransform::Fairchild => [
                0.8562, 0.3372, -0.1934,
                -0.8360, 1.8327, 0.0033,
                0.0357, -0.0469, 1.0112,
            ],
            AdaptationTransform::CMCCAT97 => [
                0.8951, -0.7502, 0.0389,
                0.2664, 1.7135, 0.0685,
                -0.1614, 0.0367, 1.0296,
            ],
            AdaptationTransform::CMCCAT2000 => [
                0.7982, 0.3389, -0.1371,
                -0.5918, 1.5512, 0.0406,
                0.0008, 0.0239, 0.9753,
            ],
            AdaptationTransform::CAT02 => [
                0.7328, 0.4296, -0.1624,
                -0.7036, 1.6975, 0.0061,
                0.0030, 0.0136, 0.9834,
            ],
            AdaptationTransform::CAT02BrillSusstrunk => [
                0.7328, 0.4296, -0.1624,
                -0.7036, 1.6975, 0.0061,
                0.0000, 0.0000, 1.0000,
            ],
            AdaptationTransform::BiancoSchettini => [
                0.8752, 0.2787, -0.1539,
                -0.8904, 1.8709, 0.0195,
                -0.0061, 0.0162, 0.9899,
            ],
            AdaptationTransform::BiancoSchettiniPC => [
                0.6489, 0.3915, -0.0404,
                -0.3775, 1.3055, 0.0720,
                -0.0271, 0.0888, 0.9383,
            ],
        }
    }
}
