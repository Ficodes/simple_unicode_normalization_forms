pub const CHAR_TO_AVOID: &'static [(char, char)] = &[
    ('\u{0000}', '\u{001F}'), // Control chars 1
    ('\u{007F}', '\u{009F}'), // Control chars 2
    ('\u{FE00}', '\u{FE0F}'), // Variation Selectors
    ('\u{20D0}', '\u{20FF}'), // Combining Diacritical Marks for Symbols
    ('\u{2800}', '\u{28FF}'), // Braille Patterns
    // ('\u{D800}', '\u{F8FF}'), // High Surrogates, High Private Use Surrogates, Low Surrogates and Private Use Area blocks
    ('\u{E000}', '\u{F8FF}'),    // Private Use Area blocks
    ('\u{10000}', '\u{10FFFF}'), // Extra planes
];

pub const EMOJI: &'static [(char, char)] = &[
    ('🏻', '🏿'),
    ('☝', '☝'),
    ('⛹', '⛹'),
    ('✊', '✍'),
    ('⌚', '⌛'),
    ('⏩', '⏬'),
    ('⏰', '⏰'),
    ('⏳', '⏳'),
    ('◽', '◾'),
    ('☔', '☕'),
    ('♈', '♓'),
    ('♿', '♿'),
    ('⚓', '⚓'),
    ('⚡', '⚡'),
    ('⚪', '⚫'),
    ('⚽', '⚾'),
    ('⛄', '⛅'),
    ('⛎', '⛎'),
    ('⛔', '⛔'),
    ('⛪', '⛪'),
    ('⛲', '⛳'),
    ('⛵', '⛵'),
    ('⛺', '⛺'),
    ('⛽', '⛽'),
    ('✅', '✅'),
    ('✊', '✋'),
    ('✨', '✨'),
    ('❌', '❌'),
    ('❎', '❎'),
    ('❓', '❕'),
    ('❗', '❗'),
    ('➕', '➗'),
    ('➰', '➰'),
    ('➿', '➿'),
    ('⬛', '⬜'),
    ('⭐', '⭐'),
    ('⭕', '⭕'),
];

pub trait IsEmoji {
    fn is_emoji(&self) -> bool;
    fn is_char_to_avoid(&self) -> bool;
}
impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        for (lc, hc) in EMOJI {
            if self >= lc && self <= hc {
                return true;
            }
        }
        false
    }

    fn is_char_to_avoid(&self) -> bool {
        for (lc, hc) in CHAR_TO_AVOID {
            if self >= lc && self <= hc {
                return true;
            }
        }
        false
    }
}
