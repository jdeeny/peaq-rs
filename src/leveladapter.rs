/* #PeaqLevelAdapter encapsulates the level and pattern adaptation described in
* section 3.1 of <xref linkend="BS1387" />. It estimates the per-band level
* differences between reference and test signal and adapts them to each other
* to compensate level differences and linear distortions.
*/

use crate::EarModel;

pub struct LevelAdapter {
}

impl LevelAdapter {
    pub fn new(earmodel: &impl EarModel) -> Self
    {
        Self { }
    }
}
