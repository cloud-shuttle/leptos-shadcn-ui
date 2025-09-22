//! Drawer component types and enums
//! 
//! This module contains the type definitions and enums used by the Drawer component.

#[derive(Debug, Clone, PartialEq)]
pub enum DrawerDirection {
    Top,
    Bottom,
    Left,
    Right,
}

impl Default for DrawerDirection {
    fn default() -> Self {
        DrawerDirection::Bottom
    }
}
