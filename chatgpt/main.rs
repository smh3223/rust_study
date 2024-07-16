#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern crate libc;

use std::time::Duration;
use std::thread::sleep;
use crossterm::{execute, terminal, cursor};
use crossterm::event::{self, Event, KeyCode};
use std::io::{stdout, Write};

extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar(__c: libc::c_int) -> libc::c_int;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ball {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub dx: libc::c_int,
    pub dy: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Paddle {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
}

#[no_mangle]
pub unsafe extern "C" fn gotoxy(mut x: libc::c_int, mut y: libc::c_int) {
    execute!(
        stdout(),
        cursor::MoveTo(x as u16, y as u16)
    ).unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn hidecursor() {
    execute!(
        stdout(),
        cursor::Hide
    ).unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn draw_paddle(mut paddle: *mut Paddle) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*paddle).width {
        gotoxy((*paddle).x + i, (*paddle).y);
        putchar('=' as i32);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn clear_paddle(mut paddle: *mut Paddle) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*paddle).width {
        gotoxy((*paddle).x + i, (*paddle).y);
        putchar(' ' as i32);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn draw_ball(mut ball: *mut Ball) {
    gotoxy((*ball).x, (*ball).y);
    putchar('O' as i32);
}

#[no_mangle]
pub unsafe extern "C" fn clear_ball(mut ball: *mut Ball) {
    gotoxy((*ball).x, (*ball).y);
    putchar(' ' as i32);
}

#[no_mangle]
pub unsafe extern "C" fn draw_bricks(mut bricks: *mut [libc::c_char; 40]) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 20 as libc::c_int / 4 as libc::c_int {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 40 as libc::c_int {
            gotoxy(j, i);
            putchar((*bricks.offset(i as isize))[j as usize] as libc::c_int);
            j += 1;
        }
        i += 1;
    }
}

unsafe fn main_0() -> libc::c_int {
    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::All)
    ).unwrap();

    hidecursor();
    let mut ball: Ball = Ball {
        x: 40 / 2,
        y: 20 / 2,
        dx: 1,
        dy: 1,
    };
    let mut paddle: Paddle = Paddle {
        x: 40 / 2 - 7 / 2,
        y: 20 - 1,
        width: 7,
    };
    let mut bricks: [[libc::c_char; 40]; 20] = [[0; 40]; 20];
    let mut i: libc::c_int = 0;
    while i < 20 / 4 {
        let mut j: libc::c_int = 0;
        while j < 40 {
            bricks[i as usize][j as usize] = '#' as i32 as libc::c_char;
            j += 1;
        }
        i += 1;
    }
    let mut score: libc::c_int = 0;
    let mut lives: libc::c_int = 3;
    let mut ch: libc::c_int;

    loop {
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::All)
        ).unwrap();
        
        clear_ball(&mut ball);
        clear_paddle(&mut paddle);
        ball.x += ball.dx;
        ball.y += ball.dy;
        if ball.x <= 0 || ball.x >= 40 - 1 {
            ball.dx = -ball.dx;
        }
        if ball.y <= 0 {
            ball.dy = -ball.dy;
        }
        if ball.y == paddle.y - 1 && ball.x >= paddle.x && ball.x < paddle.x + paddle.width {
            ball.dy = -ball.dy;
        }
        if ball.y < 20 / 4 && bricks[ball.y as usize][ball.x as usize] == '#' as i32 as libc::c_char {
            bricks[ball.y as usize][ball.x as usize] = ' ' as i32 as libc::c_char;
            ball.dy = -ball.dy;
            score += 10;
        }
        if ball.y >= 20 {
            lives -= 1;
            ball.x = 40 / 2;
            ball.y = 20 / 2;
            ball.dx = 1;
            ball.dy = 1;
            if lives == 0 {
                gotoxy(40 / 2 - 5, 20 / 2);
                printf(b"Game Over\0" as *const u8 as *const libc::c_char);
                break;
            }
        }
        if event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.code == KeyCode::Char('a') && paddle.x > 0 {
                    paddle.x -= 1;
                }
                if key_event.code == KeyCode::Char('d') && paddle.x < 40 - paddle.width {
                    paddle.x += 1;
                }
            }
        }
        draw_ball(&mut ball);
        draw_paddle(&mut paddle);
        draw_bricks(bricks.as_mut_ptr());
        gotoxy(0, 20);
        printf(b"Score: %d  Lives: %d\0" as *const u8 as *const libc::c_char, score, lives);
        sleep(Duration::from_millis(100));
    }
    0
}

pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
