use hexx::{Hex, HexLayout, Vec2};

pub const HEX_SIZE: Vec2 = Vec2::new(32.0, 18.0);

pub const ORIGIN: Vec2 = Vec2::ZERO;

pub const CENTER_HEX: Hex = Hex { x: 0, y: 0 };
pub const LAYOUT: HexLayout = HexLayout {
    hex_size: HEX_SIZE,
    orientation: hexx::HexOrientation::Flat,
    origin: ORIGIN,
    invert_y: false,
    invert_x: false,
};
