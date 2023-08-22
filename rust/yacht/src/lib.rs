use std::collections::HashSet;

pub enum SimpleCategory {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}
pub enum Category {
    Ones([u8; 5]),
    Twos([u8; 5]),
    Threes([u8; 5]),
    Fours([u8; 5]),
    Fives([u8; 5]),
    Sixes([u8; 5]),
    FullHouse([u8; 5]),
    FourOfAKind([u8; 5]),
    LittleStraight([u8; 5]),
    BigStraight([u8; 5]),
    Choice([u8; 5]),
    Yacht([u8; 5]),
}

impl Category {
    fn validate(&self) -> bool {
        match self {
            Category::Ones(_) => true,
            Category::Twos(_) => true,
            Category::Threes(_) => true,
            Category::Fours(_) => true,
            Category::Fives(_) => true,
            Category::Sixes(_) => true,
            Category::FullHouse(dice) => {
                let mut counter = [0; 6];
                for die in dice {
                    counter[*die as usize] += 1;
                }
                let mut three_count = false;
                let mut two_count = false;
                for count in counter {
                    if count == 3 {
                        three_count = true;
                    }
                    if count == 2 {
                        two_count = true;
                    }
                }
                three_count && two_count
            }
            Category::FourOfAKind(dice) => {
                let mut counter = [0; 6];
                for die in dice {
                    counter[*die as usize] += 1;
                }
                for count in counter {
                    if count == 4 {
                        return true;
                    }
                }
                false
            }
            Category::LittleStraight(dice) => dice == &[1, 2, 3, 4, 5],
            Category::BigStraight(dice) => dice == &[2, 3, 4, 5, 6],
            Category::Choice(_) => true,
            Category::Yacht(dice) => HashSet::<&u8>::from_iter(dice.iter()).len() == 1,
        }
    }
}

type Dice = [u8; 5];
pub fn score(dice: Dice, category: SimpleCategory) -> u8 {
    let category = match category {
        SimpleCategory::Ones => todo!(),
        SimpleCategory::Twos => todo!(),
        SimpleCategory::Threes => todo!(),
        SimpleCategory::Fours => todo!(),
        SimpleCategory::Fives => todo!(),
        SimpleCategory::Sixes => todo!(),
        SimpleCategory::FullHouse => todo!(),
        SimpleCategory::FourOfAKind => Category::FourOfAKind(dice),
        SimpleCategory::LittleStraight => Category::LittleStraight(dice),
        SimpleCategory::BigStraight => Category::BigStraight(dice),
        SimpleCategory::Choice => Category::Choice(dice),
        SimpleCategory::Yacht => Category::Yacht(dice),
    };
}
