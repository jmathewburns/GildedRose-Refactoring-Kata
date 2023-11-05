use std::fmt::{self, Display};

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
        for i in 0..self.items.len() {
            Self::update_item_quality(&mut self.items[i]);
        }
    }

    pub fn update_item_quality(item: &mut Item) {
        if item.name != "Aged Brie" && item.name != "Backstage passes to a TAFKAL80ETC concert"
        {
            if item.quality > 0 {
                if item.name != "Sulfuras, Hand of Ragnaros" {
                    item.quality = item.quality - 1;
                }
            }
        } else {
            if item.quality < 50 {
                item.quality = item.quality + 1;

                if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                    if item.sell_in < 11 {
                        if item.quality < 50 {
                            item.quality = item.quality + 1;
                        }
                    }

                    if item.sell_in < 6 {
                        if item.quality < 50 {
                            item.quality = item.quality + 1;
                        }
                    }
                }
            }
        }

        if item.name != "Sulfuras, Hand of Ragnaros" {
            item.sell_in = item.sell_in - 1;
        }

        if item.sell_in < 0 {
            if item.name != "Aged Brie" {
                if item.name != "Backstage passes to a TAFKAL80ETC concert" {
                    if item.quality > 0 {
                        if item.name != "Sulfuras, Hand of Ragnaros" {
                            item.quality = item.quality - 1;
                        }
                    }
                } else {
                    item.quality = item.quality - item.quality;
                }
            } else {
                if item.quality < 50 {
                    item.quality = item.quality + 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item};

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
            Item { name: "Backstage passes to a TAFKAL80ETC concert".to_string(), sell_in: 10, quality: 76 },
            Item { name: "Backstage passes to a TAFKAL80ETC concert".to_string(), sell_in: 9, quality: 76 },
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

    fn assert_eq_expected_after_update(original: Item, expected: Item) {
        /* Moving ok here */
        let mut updated = original;
        GildedRose::update_item_quality(&mut updated);

        assert_eq(&expected, &updated);
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
