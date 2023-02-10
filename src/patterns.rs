use rp2040_hal::{
    gpio::bank0::{Gpio26, Gpio27},
    pac::I2C1,
};

use crate::lotus::LotusLedMatrix;
use crate::lotus_led_hal as bsp;
use crate::mapping::*;

type Grid = [[u8; 34]; 9];

type Foo = LotusLedMatrix<
    bsp::hal::I2C<
        I2C1,
        (
            bsp::hal::gpio::Pin<Gpio26, bsp::hal::gpio::Function<bsp::hal::gpio::I2C>>,
            bsp::hal::gpio::Pin<Gpio27, bsp::hal::gpio::Function<bsp::hal::gpio::I2C>>,
        ),
    >,
>;

pub fn display_letters() -> Grid {
    let mut grid: Grid = [[0; 34]; 9];

    display_letter(26, &mut grid, CAP_L);
    display_letter(20, &mut grid, CAP_O);
    display_letter(12, &mut grid, CAP_T);
    display_letter(0, &mut grid, CAP_S);
    display_letter(5, &mut grid, CAP_U);

    grid
}

pub fn display_letter(pos: usize, grid: &mut Grid, letter: SingleDisplayData) {
    for x in 0..8 {
        for y in 0..8 {
            let val = if letter[x] & (1 << y) > 0 { 0xFF } else { 0 };
            grid[8 - x][y + pos] = val;
        }
    }
}

/// Gradient getting brighter from top to bottom
pub fn gradient() -> Grid {
    let mut grid: Grid = [[0; 34]; 9];
    for y in 0..34 {
        for x in 0..9 {
            grid[x][y] = (1 * (y + 1)) as u8;
        }
    }
    grid
}

/// Fill a percentage of the rows from the bottom up
pub fn percentage(percentage: u16) -> Grid {
    let mut grid: Grid = [[0; 34]; 9];
    let first_row = 34 * percentage / 100;
    for y in (34 - first_row)..34 {
        for x in 0..9 {
            grid[x][y as usize] = 0xFF;
        }
    }
    grid
}

/// Double sided gradient, bright in the middle, dim top and bottom
pub fn double_gradient() -> Grid {
    let mut grid: Grid = [[0; 34]; 9];
    for y in 0..(34 / 2) {
        for x in 0..9 {
            grid[x][y] = (1 * (y + 1)) as u8;
        }
    }
    for y in (34 / 2)..34 {
        for x in 0..9 {
            grid[x][y] = 34 - (1 * (y + 1)) as u8;
        }
    }
    grid
}

pub fn fill_grid(grid: Grid, matrix: &mut Foo) {
    for y in 0..34 {
        for x in 0..9 {
            matrix
                .device
                .pixel(x, y, grid[x as usize][y as usize])
                .unwrap();
        }
    }
}

pub fn fill_grid_pixels(grid: Grid, matrix: &mut Foo) {
    let mut brightnesses = [0x00; 0xB4 + 0xAB];
    for y in 0..34 {
        for x in 0..9 {
            let (register, page) = (matrix.device.calc_pixel)(x, y);
            brightnesses[(page * 0xAA + register) as usize] = grid[x as usize][y as usize];
        }
    }
    matrix.device.fill_matrix(&brightnesses).unwrap();
}

pub fn full_brightness(matrix: &mut Foo) {
    // Fills every pixel individually
    matrix.fill_brightness(0xFF).unwrap();

    // Fills full page at once
    //matrix.device.fill(0xFF).unwrap();
}

pub fn zigzag() -> Grid {
    let mut grid: Grid = [[0; 34]; 9];
    // 1st Right to left
    for i in 0..9 {
        grid[i][i] = 0xFF;
    }
    // 1st Left to right
    for i in 0..9 {
        grid[8 - i][9 + i] = 0xFF;
    }
    // 2nd right to left
    for i in 0..9 {
        grid[i][18 + i] = 0xFF;
    }
    // 2nd left to right
    for i in 0..9 {
        if 27 + i < 34 {
            grid[8 - i][27 + i] = 0xFF;
        }
    }
    grid[1][33] = 0xFF;
    grid
}
