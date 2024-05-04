use std::fmt::{self, Display};
use std::string::ToString;

pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            update_item_quality(item);
        }
    }
}

pub fn update_item_quality(item: &mut Item) {
    const AGED_BRIE: &str = "Aged Brie";
    const BACKSTAGE_PASSES: &str = "Backstage passes to a TAFKAL80ETC concert";
    const SULFURAS: &str = "Sulfuras, Hand of Ragnaros";

    let (sell_in_delta, quality_delta) = match (item.name.as_str(), item.sell_in, item.quality) {
        (SULFURAS,                      ..) => { ( 0,        0) }
        (AGED_BRIE,                     ..) => { (-1,        1) }
        (BACKSTAGE_PASSES,  ..= 0, quality) => { (-1, -quality) }
        (BACKSTAGE_PASSES, 1..= 5,      ..) => { (-1,        3) }
        (BACKSTAGE_PASSES, 6..=10,      ..) => { (-1,        2) }
        (BACKSTAGE_PASSES,              ..) => { (-1,        1) }
        (_,                  ..=0,       _) => { (-1,       -2) }
        (                               ..) => { (-1,       -1) }
    };

    item.quality += clamp_quality_delta(item.quality, quality_delta);
    item.sell_in += sell_in_delta;
}

fn clamp_quality_delta(current: i32, delta: i32) -> i32 {
    const QUALITY_MAX: i32 = 50;
    const QUALITY_MIN: i32 = 0;
    const NO_CHANGE: i32 = 0;

    if current >= QUALITY_MAX || current <= QUALITY_MIN {
        NO_CHANGE
    } else if current + delta > QUALITY_MAX {
        current + delta - QUALITY_MAX
    } else if current + delta < QUALITY_MIN {
        -current
    } else {
        delta
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item, update_item_quality};

    #[test]
    pub fn should_not_add_or_remove_items() {
        let items = vec![Item::new("foo", 10, 10)];
        let original_size = items.len();

        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(original_size, rose.items.len());
    }

    #[test]
    pub fn legendary_item_should_not_degrade_in_quality_or_sell_in_date() {
        assert_eq_expected_after_update(
            Item::new("Sulfuras, Hand of Ragnaros", 10, 80),
            Item::new("Sulfuras, Hand of Ragnaros", 10, 80),
        );
    }

    #[test]
    pub fn normal_item_quality_and_sell_in_should_drop_by_one() {
        assert_eq_expected_after_update(
            Item { name: "foo".to_string(), sell_in: 10, quality: 10 },
            Item { name: "foo".to_string(), sell_in: 9, quality: 9 },
        );
    }

    #[test]
    pub fn normal_item_quality_should_not_go_negative() {
        assert_eq_expected_after_update(
            Item { name: "foo".to_string(), sell_in: 10, quality: 0 },
            Item { name: "foo".to_string(), sell_in: 9, quality: 0 },
        );
    }

    #[test]
    pub fn normal_item_quality_should_go_down_by_two_after_sell_in_date_passes() {
        assert_eq_expected_after_update(
            Item { name: "foo".to_string(), sell_in: 0, quality: 10 },
            Item { name: "foo".to_string(), sell_in: -1, quality: 8 },
        );
    }

    #[test]
    pub fn aged_brie_quality_should_go_up_one_one() {
        assert_eq_expected_after_update(
            Item { name: "Aged Brie".to_string(), sell_in: 10, quality: 10 },
            Item { name: "Aged Brie".to_string(), sell_in: 9, quality: 11 },
        );
    }

    #[test]
    pub fn aged_brie_quality_should_not_go_above_fifty() {
        assert_eq_expected_after_update(
            Item { name: "Aged Brie".to_string(), sell_in: 10, quality: 50 },
            Item { name: "Aged Brie".to_string(), sell_in: 9, quality: 50 },
        );
    }

    #[test]
    pub fn aged_brie_quality_already_above_fifty_should_not_go_down() {
        assert_eq_expected_after_update(
            Item { name: "Aged Brie".to_string(), sell_in: 10, quality: 76 },
            Item { name: "Aged Brie".to_string(), sell_in: 9, quality: 76 },
        );
    }

    #[test]
    pub fn backstage_pass_quality_should_increase_by_two_ten_days_before_concert() {
        assert_eq_expected_after_update(
            Item { name: "Backstage passes to a TAFKAL80ETC concert".to_string(), sell_in: 10, quality: 10 },
            Item { name: "Backstage passes to a TAFKAL80ETC concert".to_string(), sell_in: 9, quality: 12 },
        );
    }

    #[test]
    pub fn backstage_pass_quality_should_increase_by_two_less_than_ten_days_before_concert() {
        assert_eq_expected_after_update(
            Item { name: "Backstage passes to a TAFKAL80ETC concert".to_string(), sell_in: 9, quality: 10 },
            Item { name: "Backstage passes to a TAFKAL80ETC concert".to_string(), sell_in: 8, quality: 12 },
        );
    }

    #[test]
    pub fn backstage_pass_quality_should_increase_by_three_five_days_before_concert() {
        assert_eq_expected_after_update(
            Item { name: "Backstage passes to a TAFKAL80ETC concert".to_string(), sell_in: 5, quality: 10 },
            Item { name: "Backstage passes to a TAFKAL80ETC concert".to_string(), sell_in: 4, quality: 13 },
        );
    }

    #[test]
    pub fn backstage_pass_quality_should_increase_by_three_less_than_five_days_before_concert() {
        assert_eq_expected_after_update(
            Item { name: "Backstage passes to a TAFKAL80ETC concert".to_string(), sell_in: 4, quality: 10 },
            Item { name: "Backstage passes to a TAFKAL80ETC concert".to_string(), sell_in: 3, quality: 13 },
        );
    }

    #[test]
    pub fn backstage_pass_quality_be_zero_after_concert() {
        assert_eq_expected_after_update(
            Item { name: "Backstage passes to a TAFKAL80ETC concert".to_string(), sell_in: 0, quality: 10 },
            Item { name: "Backstage passes to a TAFKAL80ETC concert".to_string(), sell_in: -1, quality: 0 },
        );
    }

    #[test]
    pub fn backstage_pass_quality_should_not_go_above_fifty() {
        assert_eq_expected_after_update(
            Item { name: "Backstage passes to a TAFKAL80ETC concert".to_string(), sell_in: 30, quality: 50 },
            Item { name: "Backstage passes to a TAFKAL80ETC concert".to_string(), sell_in: 29, quality: 50 },
        );
    }

    #[test]
    pub fn backstage_pass_quality_already_above_fifty_should_not_go_down() {
        assert_eq_expected_after_update(
            Item { name: "Backstage passes to a TAFKAL80ETC concert".to_string(), sell_in: 30, quality: 76 },
            Item { name: "Backstage passes to a TAFKAL80ETC concert".to_string(), sell_in: 29, quality: 76 },
        );
    }


    fn assert_eq_expected_after_update(mut original: Item, expected: Item) {
        update_item_quality(&mut original);

        assert_eq(&expected, &original);
    }

    fn assert_all_eq(one: &[Item], other: &[Item]) {
        for (one_item, other_item) in one.iter().zip(other) {
            assert_eq(one_item, other_item);
        }
    }

    fn assert_eq(one: &Item, other: &Item) {
        /* Can't change Item struct to derive Eq */
        assert_eq!(one.name, other.name);
        assert_eq!(one.quality, other.quality);
        assert_eq!(one.sell_in, other.sell_in);
    }
}
