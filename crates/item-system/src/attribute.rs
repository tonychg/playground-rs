use std::{cmp, ops};

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub enum Modifier {
    IncreaseTokenAmount,
    IncreaseItemDropRate,
    ReduceMissionDuration,
    EquipmentSlot,
    UnlockExpertMission,
    RechargeDurability,
    #[default]
    Unknown,
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Attribute {
    pub modifier: Modifier,
    pub value: u32,
    pub decimals: u32,
}

impl ops::Add<Attribute> for Attribute {
    type Output = Attribute;

    fn add(self, rhs: Attribute) -> Self::Output {
        let mut result = self;
        let mut rhs = rhs;
        let decimals = cmp::max(result.decimals, rhs.decimals);
        if result.decimals < decimals {
            result.value *= 10u32.pow(decimals);
        }
        if rhs.decimals < decimals {
            rhs.value *= 10u32.pow(decimals);
        }
        result.value += rhs.value;
        result.decimals = decimals;
        result.modifier = rhs.modifier;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_with_different_decimals() {
        let attribute1 = Attribute {
            modifier: Modifier::IncreaseTokenAmount,
            value: 10,
            decimals: 0,
        };
        let attribute2 = Attribute {
            modifier: Modifier::IncreaseTokenAmount,
            value: 10,
            decimals: 1,
        };
        let expected = Attribute {
            modifier: Modifier::IncreaseTokenAmount,
            value: 110,
            decimals: 1,
        };
        assert_eq!(expected, attribute1 + attribute2);
    }

    #[test]
    fn add_with_same_decimals() {
        let attribute1 = Attribute {
            modifier: Modifier::IncreaseTokenAmount,
            value: 10,
            decimals: 1,
        };
        let attribute2 = Attribute {
            modifier: Modifier::IncreaseTokenAmount,
            value: 10,
            decimals: 1,
        };
        let expected = Attribute {
            modifier: Modifier::IncreaseTokenAmount,
            value: 20,
            decimals: 1,
        };
        assert_eq!(expected, attribute1 + attribute2);
    }

    #[test]
    fn add_with_different_decimals_target() {
        let attribute1 = Attribute {
            modifier: Modifier::IncreaseTokenAmount,
            value: 10,
            decimals: 1,
        };
        let attribute2 = Attribute {
            modifier: Modifier::IncreaseTokenAmount,
            value: 10,
            decimals: 0,
        };
        let expected = Attribute {
            modifier: Modifier::IncreaseTokenAmount,
            value: 110,
            decimals: 1,
        };
        assert_eq!(expected, attribute1 + attribute2);
    }
}
