use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::{Str, StyleOptions, StyleError};

/// Float quantized to 4 decimal places
#[derive(Debug, Clone)]
pub struct FloatQuantized(f64);

impl Eq for FloatQuantized {}

impl PartialEq for FloatQuantized {
    fn eq(&self, other: &Self) -> bool {
        self.as_i32() == other.as_i32()
    }
}

impl Ord for FloatQuantized {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_i32().cmp(&other.as_i32())
    }
}

impl PartialOrd for FloatQuantized {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_i32().partial_cmp(&other.as_i32())
    }
}

impl Hash for FloatQuantized {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_i32().hash(state);
    }
}

impl From<f64> for FloatQuantized {
    fn from(value: f64) -> Self {
        FloatQuantized(value)
    }
}

impl From<f32> for FloatQuantized {
    fn from(value: f32) -> Self {
        FloatQuantized(value as f64)
    }
}

impl std::fmt::Display for FloatQuantized {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_f64())
    }
}

impl FloatQuantized {
    pub fn as_i32(&self) -> i32 {
        (self.0 * 10000.0).round() as i32
    }

    pub fn as_f64(&self) -> f64 {
        self.as_i32() as f64 / 10000.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Color {
    /// Rose 050: rgb(255, 241, 242)
    Rose050,
    /// Rose 100: rgb(255, 228, 230)
    Rose100,
    /// Rose 200: rgb(254, 205, 211)
    Rose200,
    /// Rose 300: rgb(253, 164, 175)
    Rose300,
    /// Rose 400: rgb(251, 113, 133)
    Rose400,
    /// Rose 500: rgb(244, 63, 94)
    Rose500,
    /// Rose 600: rgb(225, 29, 72)
    Rose600,
    /// Rose 700: rgb(190, 18, 60)
    Rose700,
    /// Rose 800: rgb(159, 18, 57)
    Rose800,
    /// Rose 900: rgb(136, 19, 55)
    Rose900,
    /// Rose 950: rgb(76, 5, 25)
    Rose950,

    /// Pink 050: rgb(253, 242, 248)
    Pink050,
    /// Pink 100: rgb(252, 231, 243)
    Pink100,
    /// Pink 200: rgb(251, 207, 232)
    Pink200,
    /// Pink 300: rgb(249, 168, 212)
    Pink300,
    /// Pink 400: rgb(244, 114, 182)
    Pink400,
    /// Pink 500: rgb(236, 72, 153)
    Pink500,
    /// Pink 600: rgb(219, 39, 119)
    Pink600,
    /// Pink 700: rgb(190, 24, 93)
    Pink700,
    /// Pink 800: rgb(157, 23, 77)
    Pink800,
    /// Pink 900: rgb(131, 24, 67)
    Pink900,
    /// Pink 950: rgb(80, 7, 36)
    Pink950,

    /// Fuchsia 050: rgb(253, 244, 255)
    Fuchsia050,
    /// Fuchsia 100: rgb(250, 232, 255)
    Fuchsia100,
    /// Fuchsia 200: rgb(245, 208, 254)
    Fuchsia200,
    /// Fuchsia 300: rgb(240, 171, 252)
    Fuchsia300,
    /// Fuchsia 400: rgb(232, 121, 249)
    Fuchsia400,
    /// Fuchsia 500: rgb(217, 70, 239)
    Fuchsia500,
    /// Fuchsia 600: rgb(192, 38, 211)
    Fuchsia600,
    /// Fuchsia 700: rgb(162, 28, 175)
    Fuchsia700,
    /// Fuchsia 800: rgb(134, 25, 143)
    Fuchsia800,
    /// Fuchsia 900: rgb(112, 26, 117)
    Fuchsia900,
    /// Fuchsia 950: rgb(74, 4, 78)
    Fuchsia950,

    /// Purple 050: rgb(250, 245, 255)
    Purple050,
    /// Purple 100: rgb(243, 232, 255)
    Purple100,
    /// Purple 200: rgb(233, 213, 255)
    Purple200,
    /// Purple 300: rgb(216, 180, 254)
    Purple300,
    /// Purple 400: rgb(192, 132, 252)
    Purple400,
    /// Purple 500: rgb(168, 85, 247)
    Purple500,
    /// Purple 600: rgb(147, 51, 234)
    Purple600,
    /// Purple 700: rgb(126, 34, 206)
    Purple700,
    /// Purple 800: rgb(107, 33, 168)
    Purple800,
    /// Purple 900: rgb(88, 28, 135)
    Purple900,
    /// Purple 950: rgb(59, 7, 100)
    Purple950,

    /// Violet 050: rgb(245, 243, 255)
    Violet050,
    /// Violet 100: rgb(237, 233, 254)
    Violet100,
    /// Violet 200: rgb(221, 214, 254)
    Violet200,
    /// Violet 300: rgb(196, 181, 253)
    Violet300,
    /// Violet 400: rgb(167, 139, 250)
    Violet400,
    /// Violet 500: rgb(139, 92, 246)
    Violet500,
    /// Violet 600: rgb(124, 58, 237)
    Violet600,
    /// Violet 700: rgb(109, 40, 217)
    Violet700,
    /// Violet 800: rgb(91, 33, 182)
    Violet800,
    /// Violet 900: rgb(76, 29, 149)
    Violet900,
    /// Violet 950: rgb(46, 16, 101)
    Violet950,

    /// Indigo 050: rgb(238, 242, 255)
    Indigo050,
    /// Indigo 100: rgb(224, 231, 255)
    Indigo100,
    /// Indigo 200: rgb(199, 210, 254)
    Indigo200,
    /// Indigo 300: rgb(165, 180, 252)
    Indigo300,
    /// Indigo 400: rgb(129, 140, 248)
    Indigo400,
    /// Indigo 500: rgb(99, 102, 241)
    Indigo500,
    /// Indigo 600: rgb(79, 70, 229)
    Indigo600,
    /// Indigo 700: rgb(67, 56, 202)
    Indigo700,
    /// Indigo 800: rgb(55, 48, 163)
    Indigo800,
    /// Indigo 900: rgb(49, 46, 129)
    Indigo900,
    /// Indigo 950: rgb(30, 27, 75)
    Indigo950,

    /// Blue 050: rgb(239, 246, 255)
    Blue050,
    /// Blue 100: rgb(219, 234, 254)
    Blue100,
    /// Blue 200: rgb(191, 219, 254)
    Blue200,
    /// Blue 300: rgb(147, 197, 253)
    Blue300,
    /// Blue 400: rgb(96, 165, 250)
    Blue400,
    /// Blue 500: rgb(59, 130, 246)
    Blue500,
    /// Blue 600: rgb(37, 99, 235)
    Blue600,
    /// Blue 700: rgb(29, 78, 216)
    Blue700,
    /// Blue 800: rgb(30, 64, 175)
    Blue800,
    /// Blue 900: rgb(30, 58, 138)
    Blue900,
    /// Blue 950: rgb(23, 37, 84)
    Blue950,

    /// Sky 050: rgb(240, 249, 255)
    Sky050,
    /// Sky 100: rgb(224, 242, 254)
    Sky100,
    /// Sky 200: rgb(186, 230, 253)
    Sky200,
    /// Sky 300: rgb(125, 211, 252)
    Sky300,
    /// Sky 400: rgb(56, 189, 248)
    Sky400,
    /// Sky 500: rgb(14, 165, 233)
    Sky500,
    /// Sky 600: rgb(2, 132, 199)
    Sky600,
    /// Sky 700: rgb(3, 105, 161)
    Sky700,
    /// Sky 800: rgb(7, 89, 133)
    Sky800,
    /// Sky 900: rgb(12, 74, 110)
    Sky900,
    /// Sky 950: rgb(8, 47, 73)
    Sky950,

    /// Cyan 050: rgb(236, 254, 255)
    Cyan050,
    /// Cyan 100: rgb(207, 250, 254)
    Cyan100,
    /// Cyan 200: rgb(165, 243, 252)
    Cyan200,
    /// Cyan 300: rgb(103, 232, 249)
    Cyan300,
    /// Cyan 400: rgb(34, 211, 238)
    Cyan400,
    /// Cyan 500: rgb(6, 182, 212)
    Cyan500,
    /// Cyan 600: rgb(8, 145, 178)
    Cyan600,
    /// Cyan 700: rgb(14, 116, 144)
    Cyan700,
    /// Cyan 800: rgb(21, 94, 117)
    Cyan800,
    /// Cyan 900: rgb(22, 78, 99)
    Cyan900,
    /// Cyan 950: rgb(8, 51, 68)
    Cyan950,

    /// Teal 050: rgb(240, 253, 250)
    Teal050,
    /// Teal 100: rgb(204, 251, 241)
    Teal100,
    /// Teal 200: rgb(153, 246, 228)
    Teal200,
    /// Teal 300: rgb(94, 234, 212)
    Teal300,
    /// Teal 400: rgb(45, 212, 191)
    Teal400,
    /// Teal 500: rgb(20, 184, 166)
    Teal500,
    /// Teal 600: rgb(13, 148, 136)
    Teal600,
    /// Teal 700: rgb(15, 118, 110)
    Teal700,
    /// Teal 800: rgb(17, 94, 89)
    Teal800,
    /// Teal 900: rgb(19, 78, 74)
    Teal900,
    /// Teal 950: rgb(4, 47, 46)
    Teal950,

    /// Emerald 050: rgb(236, 253, 245)
    Emerald050,
    /// Emerald 100: rgb(209, 250, 229)
    Emerald100,
    /// Emerald 200: rgb(167, 243, 208)
    Emerald200,
    /// Emerald 300: rgb(110, 231, 183)
    Emerald300,
    /// Emerald 400: rgb(52, 211, 153)
    Emerald400,
    /// Emerald 500: rgb(16, 185, 129)
    Emerald500,
    /// Emerald 600: rgb(5, 150, 105)
    Emerald600,
    /// Emerald 700: rgb(4, 120, 87)
    Emerald700,
    /// Emerald 800: rgb(6, 95, 70)
    Emerald800,
    /// Emerald 900: rgb(6, 78, 59)
    Emerald900,
    /// Emerald 950: rgb(2, 44, 34)
    Emerald950,

    /// Green 050: rgb(240, 253, 244)
    Green050,
    /// Green 100: rgb(220, 252, 231)
    Green100,
    /// Green 200: rgb(187, 247, 208)
    Green200,
    /// Green 300: rgb(134, 239, 172)
    Green300,
    /// Green 400: rgb(74, 222, 128)
    Green400,
    /// Green 500: rgb(34, 197, 94)
    Green500,
    /// Green 600: rgb(22, 163, 74)
    Green600,
    /// Green 700: rgb(21, 128, 61)
    Green700,
    /// Green 800: rgb(22, 101, 52)
    Green800,
    /// Green 900: rgb(20, 83, 45)
    Green900,
    /// Green 950: rgb(5, 46, 22)
    Green950,

    /// Lime 050: rgb(247, 254, 231)
    Lime050,
    /// Lime 100: rgb(236, 252, 203)
    Lime100,
    /// Lime 200: rgb(217, 249, 157)
    Lime200,
    /// Lime 300: rgb(190, 242, 100)
    Lime300,
    /// Lime 400: rgb(163, 230, 53)
    Lime400,
    /// Lime 500: rgb(132, 204, 22)
    Lime500,
    /// Lime 600: rgb(101, 163, 13)
    Lime600,
    /// Lime 700: rgb(77, 124, 15)
    Lime700,
    /// Lime 800: rgb(63, 98, 18)
    Lime800,
    /// Lime 900: rgb(54, 83, 20)
    Lime900,
    /// Lime 950: rgb(26, 46, 5)
    Lime950,

    /// Yellow 050: rgb(254, 252, 232)
    Yellow050,
    /// Yellow 100: rgb(254, 249, 195)
    Yellow100,
    /// Yellow 200: rgb(254, 240, 138)
    Yellow200,
    /// Yellow 300: rgb(253, 224, 71)
    Yellow300,
    /// Yellow 400: rgb(250, 204, 21)
    Yellow400,
    /// Yellow 500: rgb(234, 179, 8)
    Yellow500,
    /// Yellow 600: rgb(202, 138, 4)
    Yellow600,
    /// Yellow 700: rgb(161, 98, 7)
    Yellow700,
    /// Yellow 800: rgb(133, 77, 14)
    Yellow800,
    /// Yellow 900: rgb(113, 63, 18)
    Yellow900,
    /// Yellow 950: rgb(66, 32, 6)
    Yellow950,

    /// Amber 050: rgb(255, 251, 235)
    Amber050,
    /// Amber 100: rgb(254, 243, 199)
    Amber100,
    /// Amber 200: rgb(253, 230, 138)
    Amber200,
    /// Amber 300: rgb(252, 211, 77)
    Amber300,
    /// Amber 400: rgb(251, 191, 36)
    Amber400,
    /// Amber 500: rgb(245, 158, 11)
    Amber500,
    /// Amber 600: rgb(217, 119, 6)
    Amber600,
    /// Amber 700: rgb(180, 83, 9)
    Amber700,
    /// Amber 800: rgb(146, 64, 14)
    Amber800,
    /// Amber 900: rgb(120, 53, 15)
    Amber900,
    /// Amber 950: rgb(69, 26, 3)
    Amber950,

    /// Orange 050: rgb(255, 247, 237)
    Orange050,
    /// Orange 100: rgb(255, 237, 213)
    Orange100,
    /// Orange 200: rgb(254, 215, 170)
    Orange200,
    /// Orange 300: rgb(253, 186, 116)
    Orange300,
    /// Orange 400: rgb(251, 146, 60)
    Orange400,
    /// Orange 500: rgb(249, 115, 22)
    Orange500,
    /// Orange 600: rgb(234, 88, 12)
    Orange600,
    /// Orange 700: rgb(194, 65, 12)
    Orange700,
    /// Orange 800: rgb(154, 52, 18)
    Orange800,
    /// Orange 900: rgb(124, 45, 18)
    Orange900,
    /// Orange 950: rgb(67, 20, 7)
    Orange950,

    /// Red 050: rgb(254, 242, 242)
    Red050,
    /// Red 100: rgb(254, 226, 226)
    Red100,
    /// Red 200: rgb(254, 202, 202)
    Red200,
    /// Red 300: rgb(252, 165, 165)
    Red300,
    /// Red 400: rgb(248, 113, 113)
    Red400,
    /// Red 500: rgb(239, 68, 68)
    Red500,
    /// Red 600: rgb(220, 38, 38)
    Red600,
    /// Red 700: rgb(185, 28, 28)
    Red700,
    /// Red 800: rgb(153, 27, 27)
    Red800,
    /// Red 900: rgb(127, 29, 29)
    Red900,
    /// Red 950: rgb(69, 10, 10)
    Red950,

    /// Stone 050: rgb(250, 250, 249)
    Stone050,
    /// Stone 100: rgb(245, 245, 244)
    Stone100,
    /// Stone 200: rgb(231, 229, 228)
    Stone200,
    /// Stone 300: rgb(214, 211, 209)
    Stone300,
    /// Stone 400: rgb(168, 162, 158)
    Stone400,
    /// Stone 500: rgb(120, 113, 108)
    Stone500,
    /// Stone 600: rgb(87, 83, 78)
    Stone600,
    /// Stone 700: rgb(68, 64, 60)
    Stone700,
    /// Stone 800: rgb(41, 37, 36)
    Stone800,
    /// Stone 900: rgb(28, 25, 23)
    Stone900,
    /// Stone 950: rgb(12, 10, 9)
    Stone950,

    /// Zinc 050: rgb(250, 250, 250)
    Zinc050,
    /// Zinc 100: rgb(244, 244, 245)
    Zinc100,
    /// Zinc 200: rgb(228, 228, 231)
    Zinc200,
    /// Zinc 300: rgb(212, 212, 216)
    Zinc300,
    /// Zinc 400: rgb(161, 161, 170)
    Zinc400,
    /// Zinc 500: rgb(113, 113, 122)
    Zinc500,
    /// Zinc 600: rgb(82, 82, 91)
    Zinc600,
    /// Zinc 700: rgb(63, 63, 70)
    Zinc700,
    /// Zinc 800: rgb(39, 39, 42)
    Zinc800,
    /// Zinc 900: rgb(24, 24, 27)
    Zinc900,
    /// Zinc 950: rgb(9, 9, 11)
    Zinc950,

    /// Gray 050: rgb(249, 250, 251)
    Gray050,
    /// Gray 100: rgb(243, 244, 246)
    Gray100,
    /// Gray 200: rgb(229, 231, 235)
    Gray200,
    /// Gray 300: rgb(209, 213, 219)
    Gray300,
    /// Gray 400: rgb(156, 163, 175)
    Gray400,
    /// Gray 500: rgb(107, 114, 128)
    Gray500,
    /// Gray 600: rgb(75, 85, 99)
    Gray600,
    /// Gray 700: rgb(55, 65, 81)
    Gray700,
    /// Gray 800: rgb(31, 41, 55)
    Gray800,
    /// Gray 900: rgb(17, 24, 39)
    Gray900,
    /// Gray 950: rgb(3, 7, 18)
    Gray950,

    /// Slate 050: rgb(248, 250, 252)
    Slate050,
    /// Slate 100: rgb(241, 245, 249)
    Slate100,
    /// Slate 200: rgb(226, 232, 240)
    Slate200,
    /// Slate 300: rgb(203, 213, 225)
    Slate300,
    /// Slate 400: rgb(148, 163, 184)
    Slate400,
    /// Slate 500: rgb(100, 116, 139)
    Slate500,
    /// Slate 600: rgb(71, 85, 105)
    Slate600,
    /// Slate 700: rgb(51, 65, 85)
    Slate700,
    /// Slate 800: rgb(30, 41, 59)
    Slate800,
    /// Slate 900: rgb(15, 23, 42)
    Slate900,
    /// Slate 950: rgb(2, 6, 23)
    Slate950,

    /// White: rgb(255, 255, 255)
    White,
    /// Black: rgb(0, 0, 0)
    Black,
    /// Transparent: rgba(0, 0, 0, 0)
    Transparent,

    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, FloatQuantized),
    Hsl(FloatQuantized, FloatQuantized, FloatQuantized),
    Custom(String),
}

impl Color {
    pub fn rose() -> impl Iterator<Item = Color> {
        vec![
            Color::Rose050,
            Color::Rose100,
            Color::Rose200,
            Color::Rose300,
            Color::Rose400,
            Color::Rose500,
            Color::Rose600,
            Color::Rose700,
            Color::Rose800,
            Color::Rose900,
            Color::Rose950,
        ]
        .into_iter()
    }

    pub fn pink() -> impl Iterator<Item = Color> {
        vec![
            Color::Pink050,
            Color::Pink100,
            Color::Pink200,
            Color::Pink300,
            Color::Pink400,
            Color::Pink500,
            Color::Pink600,
            Color::Pink700,
            Color::Pink800,
            Color::Pink900,
            Color::Pink950,
        ]
        .into_iter()
    }

    pub fn fuchsia() -> impl Iterator<Item = Color> {
        vec![
            Color::Fuchsia050,
            Color::Fuchsia100,
            Color::Fuchsia200,
            Color::Fuchsia300,
            Color::Fuchsia400,
            Color::Fuchsia500,
            Color::Fuchsia600,
            Color::Fuchsia700,
            Color::Fuchsia800,
            Color::Fuchsia900,
            Color::Fuchsia950,
        ]
        .into_iter()
    }

    pub fn purple() -> impl Iterator<Item = Color> {
        vec![
            Color::Purple050,
            Color::Purple100,
            Color::Purple200,
            Color::Purple300,
            Color::Purple400,
            Color::Purple500,
            Color::Purple600,
            Color::Purple700,
            Color::Purple800,
            Color::Purple900,
            Color::Purple950,
        ]
        .into_iter()
    }

    pub fn violet() -> impl Iterator<Item = Color> {
        vec![
            Color::Violet050,
            Color::Violet100,
            Color::Violet200,
            Color::Violet300,
            Color::Violet400,
            Color::Violet500,
            Color::Violet600,
            Color::Violet700,
            Color::Violet800,
            Color::Violet900,
            Color::Violet950,
        ]
        .into_iter()
    }

    pub fn indigo() -> impl Iterator<Item = Color> {
        vec![
            Color::Indigo050,
            Color::Indigo100,
            Color::Indigo200,
            Color::Indigo300,
            Color::Indigo400,
            Color::Indigo500,
            Color::Indigo600,
            Color::Indigo700,
            Color::Indigo800,
            Color::Indigo900,
            Color::Indigo950,
        ]
        .into_iter()
    }

    pub fn blue() -> impl Iterator<Item = Color> {
        vec![
            Color::Blue050,
            Color::Blue100,
            Color::Blue200,
            Color::Blue300,
            Color::Blue400,
            Color::Blue500,
            Color::Blue600,
            Color::Blue700,
            Color::Blue800,
            Color::Blue900,
            Color::Blue950,
        ]
        .into_iter()
    }

    pub fn sky() -> impl Iterator<Item = Color> {
        vec![
            Color::Sky050,
            Color::Sky100,
            Color::Sky200,
            Color::Sky300,
            Color::Sky400,
            Color::Sky500,
            Color::Sky600,
            Color::Sky700,
            Color::Sky800,
            Color::Sky900,
            Color::Sky950,
        ]
        .into_iter()
    }

    pub fn cyan() -> impl Iterator<Item = Color> {
        vec![
            Color::Cyan050,
            Color::Cyan100,
            Color::Cyan200,
            Color::Cyan300,
            Color::Cyan400,
            Color::Cyan500,
            Color::Cyan600,
            Color::Cyan700,
            Color::Cyan800,
            Color::Cyan900,
            Color::Cyan950,
        ]
        .into_iter()
    }

    pub fn teal() -> impl Iterator<Item = Color> {
        vec![
            Color::Teal050,
            Color::Teal100,
            Color::Teal200,
            Color::Teal300,
            Color::Teal400,
            Color::Teal500,
            Color::Teal600,
            Color::Teal700,
            Color::Teal800,
            Color::Teal900,
            Color::Teal950,
        ]
        .into_iter()
    }

    pub fn emerald() -> impl Iterator<Item = Color> {
        vec![
            Color::Emerald050,
            Color::Emerald100,
            Color::Emerald200,
            Color::Emerald300,
            Color::Emerald400,
            Color::Emerald500,
            Color::Emerald600,
            Color::Emerald700,
            Color::Emerald800,
            Color::Emerald900,
            Color::Emerald950,
        ]
        .into_iter()
    }

    pub fn green() -> impl Iterator<Item = Color> {
        vec![
            Color::Green050,
            Color::Green100,
            Color::Green200,
            Color::Green300,
            Color::Green400,
            Color::Green500,
            Color::Green600,
            Color::Green700,
            Color::Green800,
            Color::Green900,
            Color::Green950,
        ]
        .into_iter()
    }

    pub fn lime() -> impl Iterator<Item = Color> {
        vec![
            Color::Lime050,
            Color::Lime100,
            Color::Lime200,
            Color::Lime300,
            Color::Lime400,
            Color::Lime500,
            Color::Lime600,
            Color::Lime700,
            Color::Lime800,
            Color::Lime900,
            Color::Lime950,
        ]
        .into_iter()
    }

    pub fn yellow() -> impl Iterator<Item = Color> {
        vec![
            Color::Yellow050,
            Color::Yellow100,
            Color::Yellow200,
            Color::Yellow300,
            Color::Yellow400,
            Color::Yellow500,
            Color::Yellow600,
            Color::Yellow700,
            Color::Yellow800,
            Color::Yellow900,
            Color::Yellow950,
        ]
        .into_iter()
    }

    pub fn amber() -> impl Iterator<Item = Color> {
        vec![
            Color::Amber050,
            Color::Amber100,
            Color::Amber200,
            Color::Amber300,
            Color::Amber400,
            Color::Amber500,
            Color::Amber600,
            Color::Amber700,
            Color::Amber800,
            Color::Amber900,
            Color::Amber950,
        ]
        .into_iter()
    }

    pub fn orange() -> impl Iterator<Item = Color> {
        vec![
            Color::Orange050,
            Color::Orange100,
            Color::Orange200,
            Color::Orange300,
            Color::Orange400,
            Color::Orange500,
            Color::Orange600,
            Color::Orange700,
            Color::Orange800,
            Color::Orange900,
            Color::Orange950,
        ]
        .into_iter()
    }

    pub fn red() -> impl Iterator<Item = Color> {
        vec![
            Color::Red050,
            Color::Red100,
            Color::Red200,
            Color::Red300,
            Color::Red400,
            Color::Red500,
            Color::Red600,
            Color::Red700,
            Color::Red800,
            Color::Red900,
            Color::Red950,
        ]
        .into_iter()
    }

    pub fn stone() -> impl Iterator<Item = Color> {
        vec![
            Color::Stone050,
            Color::Stone100,
            Color::Stone200,
            Color::Stone300,
            Color::Stone400,
            Color::Stone500,
            Color::Stone600,
            Color::Stone700,
            Color::Stone800,
            Color::Stone900,
            Color::Stone950,
        ]
        .into_iter()
    }

    pub fn zinc() -> impl Iterator<Item = Color> {
        vec![
            Color::Zinc050,
            Color::Zinc100,
            Color::Zinc200,
            Color::Zinc300,
            Color::Zinc400,
            Color::Zinc500,
            Color::Zinc600,
            Color::Zinc700,
            Color::Zinc800,
            Color::Zinc900,
            Color::Zinc950,
        ]
        .into_iter()
    }

    pub fn gray() -> impl Iterator<Item = Color> {
        vec![
            Color::Gray050,
            Color::Gray100,
            Color::Gray200,
            Color::Gray300,
            Color::Gray400,
            Color::Gray500,
            Color::Gray600,
            Color::Gray700,
            Color::Gray800,
            Color::Gray900,
            Color::Gray950,
        ]
        .into_iter()
    }

    pub fn slate() -> impl Iterator<Item = Color> {
        vec![
            Color::Slate050,
            Color::Slate100,
            Color::Slate200,
            Color::Slate300,
            Color::Slate400,
            Color::Slate500,
            Color::Slate600,
            Color::Slate700,
            Color::Slate800,
            Color::Slate900,
            Color::Slate950,
        ]
        .into_iter()
    }
}

impl Color {
    pub fn to_rgb(self) -> Self {
        match self {
            Self::Rose050 => Self::Rgb(255, 241, 242),
            Self::Rose100 => Self::Rgb(255, 228, 230),
            Self::Rose200 => Self::Rgb(254, 205, 211),
            Self::Rose300 => Self::Rgb(253, 164, 175),
            Self::Rose400 => Self::Rgb(251, 113, 133),
            Self::Rose500 => Self::Rgb(244, 63, 94),
            Self::Rose600 => Self::Rgb(225, 29, 72),
            Self::Rose700 => Self::Rgb(190, 18, 60),
            Self::Rose800 => Self::Rgb(159, 18, 57),
            Self::Rose900 => Self::Rgb(136, 19, 55),
            Self::Rose950 => Self::Rgb(76, 5, 25),
            Self::Pink050 => Self::Rgb(253, 242, 248),
            Self::Pink100 => Self::Rgb(252, 231, 243),
            Self::Pink200 => Self::Rgb(251, 207, 232),
            Self::Pink300 => Self::Rgb(249, 168, 212),
            Self::Pink400 => Self::Rgb(244, 114, 182),
            Self::Pink500 => Self::Rgb(236, 72, 153),
            Self::Pink600 => Self::Rgb(219, 39, 119),
            Self::Pink700 => Self::Rgb(190, 24, 93),
            Self::Pink800 => Self::Rgb(157, 23, 77),
            Self::Pink900 => Self::Rgb(131, 24, 67),
            Self::Pink950 => Self::Rgb(80, 7, 36),
            Self::Fuchsia050 => Self::Rgb(253, 244, 255),
            Self::Fuchsia100 => Self::Rgb(250, 232, 255),
            Self::Fuchsia200 => Self::Rgb(245, 208, 254),
            Self::Fuchsia300 => Self::Rgb(240, 171, 252),
            Self::Fuchsia400 => Self::Rgb(232, 121, 249),
            Self::Fuchsia500 => Self::Rgb(217, 70, 239),
            Self::Fuchsia600 => Self::Rgb(192, 38, 211),
            Self::Fuchsia700 => Self::Rgb(162, 28, 175),
            Self::Fuchsia800 => Self::Rgb(134, 25, 143),
            Self::Fuchsia900 => Self::Rgb(112, 26, 117),
            Self::Fuchsia950 => Self::Rgb(74, 4, 78),
            Self::Purple050 => Self::Rgb(250, 245, 255),
            Self::Purple100 => Self::Rgb(243, 232, 255),
            Self::Purple200 => Self::Rgb(233, 213, 255),
            Self::Purple300 => Self::Rgb(216, 180, 254),
            Self::Purple400 => Self::Rgb(192, 132, 252),
            Self::Purple500 => Self::Rgb(168, 85, 247),
            Self::Purple600 => Self::Rgb(147, 51, 234),
            Self::Purple700 => Self::Rgb(126, 34, 206),
            Self::Purple800 => Self::Rgb(107, 33, 168),
            Self::Purple900 => Self::Rgb(88, 28, 135),
            Self::Purple950 => Self::Rgb(59, 7, 100),
            Self::Violet050 => Self::Rgb(245, 243, 255),
            Self::Violet100 => Self::Rgb(237, 233, 254),
            Self::Violet200 => Self::Rgb(221, 214, 254),
            Self::Violet300 => Self::Rgb(196, 181, 253),
            Self::Violet400 => Self::Rgb(167, 139, 250),
            Self::Violet500 => Self::Rgb(139, 92, 246),
            Self::Violet600 => Self::Rgb(124, 58, 237),
            Self::Violet700 => Self::Rgb(109, 40, 217),
            Self::Violet800 => Self::Rgb(91, 33, 182),
            Self::Violet900 => Self::Rgb(76, 29, 149),
            Self::Violet950 => Self::Rgb(46, 16, 101),
            Self::Indigo050 => Self::Rgb(238, 242, 255),
            Self::Indigo100 => Self::Rgb(224, 231, 255),
            Self::Indigo200 => Self::Rgb(199, 210, 254),
            Self::Indigo300 => Self::Rgb(165, 180, 252),
            Self::Indigo400 => Self::Rgb(129, 140, 248),
            Self::Indigo500 => Self::Rgb(99, 102, 241),
            Self::Indigo600 => Self::Rgb(79, 70, 229),
            Self::Indigo700 => Self::Rgb(67, 56, 202),
            Self::Indigo800 => Self::Rgb(55, 48, 163),
            Self::Indigo900 => Self::Rgb(49, 46, 129),
            Self::Indigo950 => Self::Rgb(30, 27, 75),
            Self::Blue050 => Self::Rgb(239, 246, 255),
            Self::Blue100 => Self::Rgb(219, 234, 254),
            Self::Blue200 => Self::Rgb(191, 219, 254),
            Self::Blue300 => Self::Rgb(147, 197, 253),
            Self::Blue400 => Self::Rgb(96, 165, 250),
            Self::Blue500 => Self::Rgb(59, 130, 246),
            Self::Blue600 => Self::Rgb(37, 99, 235),
            Self::Blue700 => Self::Rgb(29, 78, 216),
            Self::Blue800 => Self::Rgb(30, 64, 175),
            Self::Blue900 => Self::Rgb(30, 58, 138),
            Self::Blue950 => Self::Rgb(23, 37, 84),
            Self::Sky050 => Self::Rgb(240, 249, 255),
            Self::Sky100 => Self::Rgb(224, 242, 254),
            Self::Sky200 => Self::Rgb(186, 230, 253),
            Self::Sky300 => Self::Rgb(125, 211, 252),
            Self::Sky400 => Self::Rgb(56, 189, 248),
            Self::Sky500 => Self::Rgb(14, 165, 233),
            Self::Sky600 => Self::Rgb(2, 132, 199),
            Self::Sky700 => Self::Rgb(3, 105, 161),
            Self::Sky800 => Self::Rgb(7, 89, 133),
            Self::Sky900 => Self::Rgb(12, 74, 110),
            Self::Sky950 => Self::Rgb(8, 47, 73),
            Self::Cyan050 => Self::Rgb(236, 254, 255),
            Self::Cyan100 => Self::Rgb(207, 250, 254),
            Self::Cyan200 => Self::Rgb(165, 243, 252),
            Self::Cyan300 => Self::Rgb(103, 232, 249),
            Self::Cyan400 => Self::Rgb(34, 211, 238),
            Self::Cyan500 => Self::Rgb(6, 182, 212),
            Self::Cyan600 => Self::Rgb(8, 145, 178),
            Self::Cyan700 => Self::Rgb(14, 116, 144),
            Self::Cyan800 => Self::Rgb(21, 94, 117),
            Self::Cyan900 => Self::Rgb(22, 78, 99),
            Self::Cyan950 => Self::Rgb(8, 51, 68),
            Self::Teal050 => Self::Rgb(240, 253, 250),
            Self::Teal100 => Self::Rgb(204, 251, 241),
            Self::Teal200 => Self::Rgb(153, 246, 228),
            Self::Teal300 => Self::Rgb(94, 234, 212),
            Self::Teal400 => Self::Rgb(45, 212, 191),
            Self::Teal500 => Self::Rgb(20, 184, 166),
            Self::Teal600 => Self::Rgb(13, 148, 136),
            Self::Teal700 => Self::Rgb(15, 118, 110),
            Self::Teal800 => Self::Rgb(17, 94, 89),
            Self::Teal900 => Self::Rgb(19, 78, 74),
            Self::Teal950 => Self::Rgb(4, 47, 46),
            Self::Emerald050 => Self::Rgb(236, 253, 245),
            Self::Emerald100 => Self::Rgb(209, 250, 229),
            Self::Emerald200 => Self::Rgb(167, 243, 208),
            Self::Emerald300 => Self::Rgb(110, 231, 183),
            Self::Emerald400 => Self::Rgb(52, 211, 153),
            Self::Emerald500 => Self::Rgb(16, 185, 129),
            Self::Emerald600 => Self::Rgb(5, 150, 105),
            Self::Emerald700 => Self::Rgb(4, 120, 87),
            Self::Emerald800 => Self::Rgb(6, 95, 70),
            Self::Emerald900 => Self::Rgb(6, 78, 59),
            Self::Emerald950 => Self::Rgb(2, 44, 34),
            Self::Green050 => Self::Rgb(240, 253, 244),
            Self::Green100 => Self::Rgb(220, 252, 231),
            Self::Green200 => Self::Rgb(187, 247, 208),
            Self::Green300 => Self::Rgb(134, 239, 172),
            Self::Green400 => Self::Rgb(74, 222, 128),
            Self::Green500 => Self::Rgb(34, 197, 94),
            Self::Green600 => Self::Rgb(22, 163, 74),
            Self::Green700 => Self::Rgb(21, 128, 61),
            Self::Green800 => Self::Rgb(22, 101, 52),
            Self::Green900 => Self::Rgb(20, 83, 45),
            Self::Green950 => Self::Rgb(5, 46, 22),
            Self::Lime050 => Self::Rgb(247, 254, 231),
            Self::Lime100 => Self::Rgb(236, 252, 203),
            Self::Lime200 => Self::Rgb(217, 249, 157),
            Self::Lime300 => Self::Rgb(190, 242, 100),
            Self::Lime400 => Self::Rgb(163, 230, 53),
            Self::Lime500 => Self::Rgb(132, 204, 22),
            Self::Lime600 => Self::Rgb(101, 163, 13),
            Self::Lime700 => Self::Rgb(77, 124, 15),
            Self::Lime800 => Self::Rgb(63, 98, 18),
            Self::Lime900 => Self::Rgb(54, 83, 20),
            Self::Lime950 => Self::Rgb(26, 46, 5),
            Self::Yellow050 => Self::Rgb(254, 252, 232),
            Self::Yellow100 => Self::Rgb(254, 249, 195),
            Self::Yellow200 => Self::Rgb(254, 240, 138),
            Self::Yellow300 => Self::Rgb(253, 224, 71),
            Self::Yellow400 => Self::Rgb(250, 204, 21),
            Self::Yellow500 => Self::Rgb(234, 179, 8),
            Self::Yellow600 => Self::Rgb(202, 138, 4),
            Self::Yellow700 => Self::Rgb(161, 98, 7),
            Self::Yellow800 => Self::Rgb(133, 77, 14),
            Self::Yellow900 => Self::Rgb(113, 63, 18),
            Self::Yellow950 => Self::Rgb(66, 32, 6),
            Self::Amber050 => Self::Rgb(255, 251, 235),
            Self::Amber100 => Self::Rgb(254, 243, 199),
            Self::Amber200 => Self::Rgb(253, 230, 138),
            Self::Amber300 => Self::Rgb(252, 211, 77),
            Self::Amber400 => Self::Rgb(251, 191, 36),
            Self::Amber500 => Self::Rgb(245, 158, 11),
            Self::Amber600 => Self::Rgb(217, 119, 6),
            Self::Amber700 => Self::Rgb(180, 83, 9),
            Self::Amber800 => Self::Rgb(146, 64, 14),
            Self::Amber900 => Self::Rgb(120, 53, 15),
            Self::Amber950 => Self::Rgb(69, 26, 3),
            Self::Orange050 => Self::Rgb(255, 247, 237),
            Self::Orange100 => Self::Rgb(255, 237, 213),
            Self::Orange200 => Self::Rgb(254, 215, 170),
            Self::Orange300 => Self::Rgb(253, 186, 116),
            Self::Orange400 => Self::Rgb(251, 146, 60),
            Self::Orange500 => Self::Rgb(249, 115, 22),
            Self::Orange600 => Self::Rgb(234, 88, 12),
            Self::Orange700 => Self::Rgb(194, 65, 12),
            Self::Orange800 => Self::Rgb(154, 52, 18),
            Self::Orange900 => Self::Rgb(124, 45, 18),
            Self::Orange950 => Self::Rgb(67, 20, 7),
            Self::Red050 => Self::Rgb(254, 242, 242),
            Self::Red100 => Self::Rgb(254, 226, 226),
            Self::Red200 => Self::Rgb(254, 202, 202),
            Self::Red300 => Self::Rgb(252, 165, 165),
            Self::Red400 => Self::Rgb(248, 113, 113),
            Self::Red500 => Self::Rgb(239, 68, 68),
            Self::Red600 => Self::Rgb(220, 38, 38),
            Self::Red700 => Self::Rgb(185, 28, 28),
            Self::Red800 => Self::Rgb(153, 27, 27),
            Self::Red900 => Self::Rgb(127, 29, 29),
            Self::Red950 => Self::Rgb(69, 10, 10),
            Self::Stone050 => Self::Rgb(250, 250, 249),
            Self::Stone100 => Self::Rgb(245, 245, 244),
            Self::Stone200 => Self::Rgb(231, 229, 228),
            Self::Stone300 => Self::Rgb(214, 211, 209),
            Self::Stone400 => Self::Rgb(168, 162, 158),
            Self::Stone500 => Self::Rgb(120, 113, 108),
            Self::Stone600 => Self::Rgb(87, 83, 78),
            Self::Stone700 => Self::Rgb(68, 64, 60),
            Self::Stone800 => Self::Rgb(41, 37, 36),
            Self::Stone900 => Self::Rgb(28, 25, 23),
            Self::Stone950 => Self::Rgb(12, 10, 9),
            Self::Zinc050 => Self::Rgb(250, 250, 250),
            Self::Zinc100 => Self::Rgb(244, 244, 245),
            Self::Zinc200 => Self::Rgb(228, 228, 231),
            Self::Zinc300 => Self::Rgb(212, 212, 216),
            Self::Zinc400 => Self::Rgb(161, 161, 170),
            Self::Zinc500 => Self::Rgb(113, 113, 122),
            Self::Zinc600 => Self::Rgb(82, 82, 91),
            Self::Zinc700 => Self::Rgb(63, 63, 70),
            Self::Zinc800 => Self::Rgb(39, 39, 42),
            Self::Zinc900 => Self::Rgb(24, 24, 27),
            Self::Zinc950 => Self::Rgb(9, 9, 11),
            Self::Gray050 => Self::Rgb(249, 250, 251),
            Self::Gray100 => Self::Rgb(243, 244, 246),
            Self::Gray200 => Self::Rgb(229, 231, 235),
            Self::Gray300 => Self::Rgb(209, 213, 219),
            Self::Gray400 => Self::Rgb(156, 163, 175),
            Self::Gray500 => Self::Rgb(107, 114, 128),
            Self::Gray600 => Self::Rgb(75, 85, 99),
            Self::Gray700 => Self::Rgb(55, 65, 81),
            Self::Gray800 => Self::Rgb(31, 41, 55),
            Self::Gray900 => Self::Rgb(17, 24, 39),
            Self::Gray950 => Self::Rgb(3, 7, 18),
            Self::Slate050 => Self::Rgb(248, 250, 252),
            Self::Slate100 => Self::Rgb(241, 245, 249),
            Self::Slate200 => Self::Rgb(226, 232, 240),
            Self::Slate300 => Self::Rgb(203, 213, 225),
            Self::Slate400 => Self::Rgb(148, 163, 184),
            Self::Slate500 => Self::Rgb(100, 116, 139),
            Self::Slate600 => Self::Rgb(71, 85, 105),
            Self::Slate700 => Self::Rgb(51, 65, 85),
            Self::Slate800 => Self::Rgb(30, 41, 59),
            Self::Slate900 => Self::Rgb(15, 23, 42),
            Self::Slate950 => Self::Rgb(2, 6, 23),
            Self::White => Self::Rgb(255, 255, 255),
            Self::Black => Self::Rgb(0, 0, 0),
            Self::Transparent => Self::Rgb(0, 0, 0),
            _ => self,
        }
    }

    pub fn alpha(self, alpha: f64) -> Self {
        let alpha = FloatQuantized::from(alpha);
        match self.to_rgb() {
            Color::Rgb(r, g, b) => Color::Rgba(r, g, b, alpha),
            Color::Rgba(r, g, b, _) => Color::Rgba(r, g, b, alpha),
            x => x,
        }
    }

    pub fn to_classname(&self) -> Str {
        match self {
            Self::Rose050 => "rose-50".into(),
            Self::Rose100 => "rose-100".into(),
            Self::Rose200 => "rose-200".into(),
            Self::Rose300 => "rose-300".into(),
            Self::Rose400 => "rose-400".into(),
            Self::Rose500 => "rose-500".into(),
            Self::Rose600 => "rose-600".into(),
            Self::Rose700 => "rose-700".into(),
            Self::Rose800 => "rose-800".into(),
            Self::Rose900 => "rose-900".into(),
            Self::Rose950 => "rose-950".into(),
            Self::Pink050 => "pink-50".into(),
            Self::Pink100 => "pink-100".into(),
            Self::Pink200 => "pink-200".into(),
            Self::Pink300 => "pink-300".into(),
            Self::Pink400 => "pink-400".into(),
            Self::Pink500 => "pink-500".into(),
            Self::Pink600 => "pink-600".into(),
            Self::Pink700 => "pink-700".into(),
            Self::Pink800 => "pink-800".into(),
            Self::Pink900 => "pink-900".into(),
            Self::Pink950 => "pink-950".into(),
            Self::Fuchsia050 => "fuchsia-50".into(),
            Self::Fuchsia100 => "fuchsia-100".into(),
            Self::Fuchsia200 => "fuchsia-200".into(),
            Self::Fuchsia300 => "fuchsia-300".into(),
            Self::Fuchsia400 => "fuchsia-400".into(),
            Self::Fuchsia500 => "fuchsia-500".into(),
            Self::Fuchsia600 => "fuchsia-600".into(),
            Self::Fuchsia700 => "fuchsia-700".into(),
            Self::Fuchsia800 => "fuchsia-800".into(),
            Self::Fuchsia900 => "fuchsia-900".into(),
            Self::Fuchsia950 => "fuchsia-950".into(),
            Self::Purple050 => "purple-50".into(),
            Self::Purple100 => "purple-100".into(),
            Self::Purple200 => "purple-200".into(),
            Self::Purple300 => "purple-300".into(),
            Self::Purple400 => "purple-400".into(),
            Self::Purple500 => "purple-500".into(),
            Self::Purple600 => "purple-600".into(),
            Self::Purple700 => "purple-700".into(),
            Self::Purple800 => "purple-800".into(),
            Self::Purple900 => "purple-900".into(),
            Self::Purple950 => "purple-950".into(),
            Self::Violet050 => "violet-50".into(),
            Self::Violet100 => "violet-100".into(),
            Self::Violet200 => "violet-200".into(),
            Self::Violet300 => "violet-300".into(),
            Self::Violet400 => "violet-400".into(),
            Self::Violet500 => "violet-500".into(),
            Self::Violet600 => "violet-600".into(),
            Self::Violet700 => "violet-700".into(),
            Self::Violet800 => "violet-800".into(),
            Self::Violet900 => "violet-900".into(),
            Self::Violet950 => "violet-950".into(),
            Self::Indigo050 => "indigo-50".into(),
            Self::Indigo100 => "indigo-100".into(),
            Self::Indigo200 => "indigo-200".into(),
            Self::Indigo300 => "indigo-300".into(),
            Self::Indigo400 => "indigo-400".into(),
            Self::Indigo500 => "indigo-500".into(),
            Self::Indigo600 => "indigo-600".into(),
            Self::Indigo700 => "indigo-700".into(),
            Self::Indigo800 => "indigo-800".into(),
            Self::Indigo900 => "indigo-900".into(),
            Self::Indigo950 => "indigo-950".into(),
            Self::Blue050 => "blue-50".into(),
            Self::Blue100 => "blue-100".into(),
            Self::Blue200 => "blue-200".into(),
            Self::Blue300 => "blue-300".into(),
            Self::Blue400 => "blue-400".into(),
            Self::Blue500 => "blue-500".into(),
            Self::Blue600 => "blue-600".into(),
            Self::Blue700 => "blue-700".into(),
            Self::Blue800 => "blue-800".into(),
            Self::Blue900 => "blue-900".into(),
            Self::Blue950 => "blue-950".into(),
            Self::Sky050 => "sky-50".into(),
            Self::Sky100 => "sky-100".into(),
            Self::Sky200 => "sky-200".into(),
            Self::Sky300 => "sky-300".into(),
            Self::Sky400 => "sky-400".into(),
            Self::Sky500 => "sky-500".into(),
            Self::Sky600 => "sky-600".into(),
            Self::Sky700 => "sky-700".into(),
            Self::Sky800 => "sky-800".into(),
            Self::Sky900 => "sky-900".into(),
            Self::Sky950 => "sky-950".into(),
            Self::Cyan050 => "cyan-50".into(),
            Self::Cyan100 => "cyan-100".into(),
            Self::Cyan200 => "cyan-200".into(),
            Self::Cyan300 => "cyan-300".into(),
            Self::Cyan400 => "cyan-400".into(),
            Self::Cyan500 => "cyan-500".into(),
            Self::Cyan600 => "cyan-600".into(),
            Self::Cyan700 => "cyan-700".into(),
            Self::Cyan800 => "cyan-800".into(),
            Self::Cyan900 => "cyan-900".into(),
            Self::Cyan950 => "cyan-950".into(),
            Self::Teal050 => "teal-50".into(),
            Self::Teal100 => "teal-100".into(),
            Self::Teal200 => "teal-200".into(),
            Self::Teal300 => "teal-300".into(),
            Self::Teal400 => "teal-400".into(),
            Self::Teal500 => "teal-500".into(),
            Self::Teal600 => "teal-600".into(),
            Self::Teal700 => "teal-700".into(),
            Self::Teal800 => "teal-800".into(),
            Self::Teal900 => "teal-900".into(),
            Self::Teal950 => "teal-950".into(),
            Self::Emerald050 => "emerald-50".into(),
            Self::Emerald100 => "emerald-100".into(),
            Self::Emerald200 => "emerald-200".into(),
            Self::Emerald300 => "emerald-300".into(),
            Self::Emerald400 => "emerald-400".into(),
            Self::Emerald500 => "emerald-500".into(),
            Self::Emerald600 => "emerald-600".into(),
            Self::Emerald700 => "emerald-700".into(),
            Self::Emerald800 => "emerald-800".into(),
            Self::Emerald900 => "emerald-900".into(),
            Self::Emerald950 => "emerald-950".into(),
            Self::Green050 => "green-50".into(),
            Self::Green100 => "green-100".into(),
            Self::Green200 => "green-200".into(),
            Self::Green300 => "green-300".into(),
            Self::Green400 => "green-400".into(),
            Self::Green500 => "green-500".into(),
            Self::Green600 => "green-600".into(),
            Self::Green700 => "green-700".into(),
            Self::Green800 => "green-800".into(),
            Self::Green900 => "green-900".into(),
            Self::Green950 => "green-950".into(),
            Self::Lime050 => "lime-50".into(),
            Self::Lime100 => "lime-100".into(),
            Self::Lime200 => "lime-200".into(),
            Self::Lime300 => "lime-300".into(),
            Self::Lime400 => "lime-400".into(),
            Self::Lime500 => "lime-500".into(),
            Self::Lime600 => "lime-600".into(),
            Self::Lime700 => "lime-700".into(),
            Self::Lime800 => "lime-800".into(),
            Self::Lime900 => "lime-900".into(),
            Self::Lime950 => "lime-950".into(),
            Self::Yellow050 => "yellow-50".into(),
            Self::Yellow100 => "yellow-100".into(),
            Self::Yellow200 => "yellow-200".into(),
            Self::Yellow300 => "yellow-300".into(),
            Self::Yellow400 => "yellow-400".into(),
            Self::Yellow500 => "yellow-500".into(),
            Self::Yellow600 => "yellow-600".into(),
            Self::Yellow700 => "yellow-700".into(),
            Self::Yellow800 => "yellow-800".into(),
            Self::Yellow900 => "yellow-900".into(),
            Self::Yellow950 => "yellow-950".into(),
            Self::Amber050 => "amber-50".into(),
            Self::Amber100 => "amber-100".into(),
            Self::Amber200 => "amber-200".into(),
            Self::Amber300 => "amber-300".into(),
            Self::Amber400 => "amber-400".into(),
            Self::Amber500 => "amber-500".into(),
            Self::Amber600 => "amber-600".into(),
            Self::Amber700 => "amber-700".into(),
            Self::Amber800 => "amber-800".into(),
            Self::Amber900 => "amber-900".into(),
            Self::Amber950 => "amber-950".into(),
            Self::Orange050 => "orange-50".into(),
            Self::Orange100 => "orange-100".into(),
            Self::Orange200 => "orange-200".into(),
            Self::Orange300 => "orange-300".into(),
            Self::Orange400 => "orange-400".into(),
            Self::Orange500 => "orange-500".into(),
            Self::Orange600 => "orange-600".into(),
            Self::Orange700 => "orange-700".into(),
            Self::Orange800 => "orange-800".into(),
            Self::Orange900 => "orange-900".into(),
            Self::Orange950 => "orange-950".into(),
            Self::Red050 => "red-50".into(),
            Self::Red100 => "red-100".into(),
            Self::Red200 => "red-200".into(),
            Self::Red300 => "red-300".into(),
            Self::Red400 => "red-400".into(),
            Self::Red500 => "red-500".into(),
            Self::Red600 => "red-600".into(),
            Self::Red700 => "red-700".into(),
            Self::Red800 => "red-800".into(),
            Self::Red900 => "red-900".into(),
            Self::Red950 => "red-950".into(),
            Self::Stone050 => "stone-50".into(),
            Self::Stone100 => "stone-100".into(),
            Self::Stone200 => "stone-200".into(),
            Self::Stone300 => "stone-300".into(),
            Self::Stone400 => "stone-400".into(),
            Self::Stone500 => "stone-500".into(),
            Self::Stone600 => "stone-600".into(),
            Self::Stone700 => "stone-700".into(),
            Self::Stone800 => "stone-800".into(),
            Self::Stone900 => "stone-900".into(),
            Self::Stone950 => "stone-950".into(),
            Self::Zinc050 => "zinc-50".into(),
            Self::Zinc100 => "zinc-100".into(),
            Self::Zinc200 => "zinc-200".into(),
            Self::Zinc300 => "zinc-300".into(),
            Self::Zinc400 => "zinc-400".into(),
            Self::Zinc500 => "zinc-500".into(),
            Self::Zinc600 => "zinc-600".into(),
            Self::Zinc700 => "zinc-700".into(),
            Self::Zinc800 => "zinc-800".into(),
            Self::Zinc900 => "zinc-900".into(),
            Self::Zinc950 => "zinc-950".into(),
            Self::Gray050 => "gray-50".into(),
            Self::Gray100 => "gray-100".into(),
            Self::Gray200 => "gray-200".into(),
            Self::Gray300 => "gray-300".into(),
            Self::Gray400 => "gray-400".into(),
            Self::Gray500 => "gray-500".into(),
            Self::Gray600 => "gray-600".into(),
            Self::Gray700 => "gray-700".into(),
            Self::Gray800 => "gray-800".into(),
            Self::Gray900 => "gray-900".into(),
            Self::Gray950 => "gray-950".into(),
            Self::Slate050 => "slate-50".into(),
            Self::Slate100 => "slate-100".into(),
            Self::Slate200 => "slate-200".into(),
            Self::Slate300 => "slate-300".into(),
            Self::Slate400 => "slate-400".into(),
            Self::Slate500 => "slate-500".into(),
            Self::Slate600 => "slate-600".into(),
            Self::Slate700 => "slate-700".into(),
            Self::Slate800 => "slate-800".into(),
            Self::Slate900 => "slate-900".into(),
            Self::Slate950 => "slate-950".into(),

            Self::White => "white".into(),
            Self::Black => "black".into(),
            Self::Transparent => "transparent".into(),

            Self::Rgb(r, g, b) => Str::from(format!("rgb-{}-{}-{}", r, g, b)),
            Self::Rgba(r, g, b, a) => Str::from(format!("rgba-{}-{}-{}-{}", r, g, b, a.as_i32())),
            Self::Hsl(h, s, l) => Str::from(format!("hsl-{}-{}-{}", h, s, l)),
            Self::Custom(s) => {
                let mut hasher = DefaultHasher::new();
                s.hash(&mut hasher);
                let hash = hasher.finish();
                Str::from(format!("custom-{}", hash))
            }
        }
    }

    pub fn to_css_value(&self) -> Str {
        let color = self.clone().to_rgb();
        match color {
            Color::Rgb(r, g, b) => Str::from(format!("rgb({},{},{})", r, g, b)),
            Color::Rgba(r, g, b, a) => Str::from(format!("rgba({},{},{},{})", r, g, b, a)),
            Color::Hsl(h, s, l) => Str::from(format!("hsl({},{}%,{}%)", h, s, l)),
            Color::Custom(s) => Str::from(s),
            _ => unreachable!(),
        }
    }

    pub(crate) fn write_color_name(&self, stream: &mut String) -> Result<(), StyleError> {
        stream.push_str(&self.to_classname());
        Ok(())
    }

    pub(crate) fn write_css_value<T: StyleOptions>(
        &self,
        stream: &mut String,
        _options: &T,
    ) -> Result<(), StyleError> {
        stream.push_str(&self.to_css_value());
        Ok(())
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb(r, g, b)
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: impl Into<FloatQuantized>) -> Self {
        Self::Rgba(r, g, b, a.into())
    }
}

// TODO(julgodis): Remove?
impl From<&Color> for Color {
    fn from(color: &Color) -> Self {
        color.clone()
    }
}
