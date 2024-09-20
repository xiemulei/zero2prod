use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    /// 如果输入满足我们对订阅者姓名的所有验证约束，则返回 `SubscriberName` 实例
    /// 否则，它会抛出一个 panic
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        // `.trim()` 返回一个 `s` 的视图，不包含头部和尾部的空白字符
        // `.is_empty()` 检视视图是否包含任何字符
        let is_empty_or_whitespace = s.trim().is_empty();

        // 检查字符串是否超过256个Unicode字符
        let is_too_long = s.graphemes(true).count() > 256;

        // 定义禁止的字符列表
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        // 检查字符串是否包含任何禁止的字符
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid subscriber name.", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for SubscriberName {
    /// 暴露订阅者姓名
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "a".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Ursula Le Guin".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}
