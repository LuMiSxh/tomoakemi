use wasm_bindgen_test::*;

use tomo::chip8::{DISPLAY_HEIGHT, DISPLAY_WIDTH, OPCODE_SIZE};
use tomo::chip8::processor::{Key, Processor, Register};

const START_PC: u16 = 0xF00;
const NEXT_PC: u16 = START_PC + OPCODE_SIZE as u16;
const SKIPPED_PC: u16 = START_PC + (2 * OPCODE_SIZE as u16);

fn build_processor() -> Processor {
    let mut processor = Processor::new();
    processor.pc = START_PC as u16;
    processor.test_set_registers(0, 0);
    processor.test_set_registers(1, 0);
    processor.test_set_registers(2, 1);
    processor.test_set_registers(3, 1);
    processor.test_set_registers(4, 2);
    processor.test_set_registers(5, 2);
    processor.test_set_registers(6, 3);
    processor.test_set_registers(7, 3);
    processor.test_set_registers(8, 4);
    processor.test_set_registers(9, 4);
    processor.test_set_registers(10, 5);
    processor.test_set_registers(11, 5);
    processor.test_set_registers(12, 6);
    processor.test_set_registers(13, 6);
    processor.test_set_registers(14, 7);
    processor.test_set_registers(15, 7);
    processor
}

#[wasm_bindgen_test]
fn test_initial_state() {
    let processor = Processor::new();
    assert_eq!(processor.pc, 0x200);
    assert_eq!(processor.sp, 0);
}
#[wasm_bindgen_test]
fn test_load_data() {
    let mut processor = build_processor();
    processor.load(vec![1, 2, 3]);
    assert_eq!(processor.test_get_ram(0x200), 1);
    assert_eq!(processor.test_get_ram(0x201), 2);
    assert_eq!(processor.test_get_ram(0x202), 3);
}

// CLS
#[wasm_bindgen_test]
fn test_op_00e0() {
    let mut processor = build_processor();
    processor.execute(0x00e0);

    for y in 0..DISPLAY_HEIGHT {
        for x in 0..DISPLAY_WIDTH {
            assert_eq!(processor.display.get_pixel(y, x), false);
        }
    }
    assert_eq!(processor.pc, NEXT_PC);
}
// RET
#[wasm_bindgen_test]
fn test_op_00ee() {
    let mut processor = build_processor();
    processor.sp = 5;
    processor.test_set_stack(4, 0x6666);
    processor.execute(0x00ee);
    assert_eq!(processor.sp, 4);
    assert_eq!(processor.pc, 0x6666);
}
// JP
#[wasm_bindgen_test]
fn test_op_1nnn() {
    let mut processor = build_processor();
    processor.execute(0x1666);
    assert_eq!(processor.pc, 0x0666);
}
// CALL
#[wasm_bindgen_test]
fn test_op_2nnn() {
    let mut processor = build_processor();
    processor.execute(0x2666);
    assert_eq!(processor.pc, 0x0666);
    assert_eq!(processor.sp, 1);
    assert_eq!(processor.test_get_stack(0), NEXT_PC);
}
// SE VX, byte
#[wasm_bindgen_test]
fn test_op_3xkk() {
    let mut processor = build_processor();
    processor.execute(0x3201);
    assert_eq!(processor.pc, SKIPPED_PC);
    let mut processor = build_processor();
    processor.execute(0x3200);
    assert_eq!(processor.pc, NEXT_PC);
}
// SNE VX, byte
#[wasm_bindgen_test]
fn test_op_4xkk() {
    let mut processor = build_processor();
    processor.execute(0x4200);
    assert_eq!(processor.pc, SKIPPED_PC);
    let mut processor = build_processor();
    processor.execute(0x4201);
    assert_eq!(processor.pc, NEXT_PC);
}
// SE VX, VY
#[wasm_bindgen_test]
fn test_op_5xy0() {
    let mut processor = build_processor();
    processor.execute(0x5540);
    assert_eq!(processor.pc, SKIPPED_PC);
    let mut processor = build_processor();
    processor.execute(0x5500);
    assert_eq!(processor.pc, NEXT_PC);
}
// LD Vx, byte
#[wasm_bindgen_test]
fn test_op_6xkk() {
    let mut processor = build_processor();
    processor.execute(0x65ff);
    assert_eq!(processor.test_get_registers(5), 0xff);
    assert_eq!(processor.pc, NEXT_PC);
}
// ADD Vx, byte
#[wasm_bindgen_test]
fn test_op_7xkk() {
    let mut processor = build_processor();
    processor.execute(0x75f0);
    assert_eq!(processor.test_get_registers(5), 0xf2);
    assert_eq!(processor.pc, NEXT_PC);
}
// LD Vx, Vy
#[wasm_bindgen_test]
fn test_op_8xy0() {
    let mut processor = build_processor();
    processor.execute(0x8050);
    assert_eq!(processor.test_get_registers(0), 0x02);
    assert_eq!(processor.pc, NEXT_PC);
}

fn check_math(v1: u8, v2: u8, op: u16, result: u8, vf: u8) {
    let mut processor = build_processor();
    processor.test_set_registers(0, v1);
    processor.test_set_registers(1, v2);
    processor.test_set_registers(Register::VF as usize, 0);
    processor.execute(0x8010 + op);
    assert_eq!(processor.test_get_registers(0), result);
    assert_eq!(processor.test_get_registers(Register::VF as usize), vf);
    assert_eq!(processor.pc, NEXT_PC);
}
// OR Vx, Vy
#[wasm_bindgen_test]
fn test_op_8xy1() {
    // 0x0F or 0xF0 == 0xFF
    check_math(0x0F, 0xF0, 1, 0xFF, 0);
}
// AND Vx, Vy
#[wasm_bindgen_test]
fn test_op_8xy2() {
    // 0x0F and 0xFF == 0x0F
    check_math(0x0F, 0xFF, 2, 0x0F, 0);
}
// XOR Vx, Vy
#[wasm_bindgen_test]
fn test_op_8xy3() {
    // 0x0F xor 0xFF == 0xF0
    check_math(0x0F, 0xFF, 3, 0xF0, 0);
}
// ADD Vx, Vy
#[wasm_bindgen_test]
fn test_op_8xy4() {
    check_math(0x0F, 0x0F, 4, 0x1E, 0);
    check_math(0xFF, 0xFF, 4, 0xFE, 1);
}
// SUB Vx, Vy
#[wasm_bindgen_test]
fn test_op_8xy5() {
    check_math(0x0F, 0x01, 5, 0x0E, 1);
    check_math(0x0F, 0xFF, 5, 0x10, 0);
}
// SHR Vx
#[wasm_bindgen_test]
fn test_op_8x06() {
    // 4 >> 1 == 2
    check_math(0x04, 0, 6, 0x02, 0);
    // 5 >> 1 == 2 mit carry
    check_math(0x05, 0, 6, 0x02, 1);
}
// SUBN Vx, Vy
#[wasm_bindgen_test]
fn test_op_8xy7() {
    check_math(0x01, 0x0F, 7, 0x0E, 1);
    check_math(0xFF, 0x0F, 7, 0x10, 0);
}

// SHL Vx
#[wasm_bindgen_test]
fn test_op_8x0e() {
    check_math(0b11000000, 0, 0x0e, 0b10000000, 1);
    check_math(0b00000111, 0, 0x0e, 0b00001110, 0);
}

// SNE VX, VY
#[wasm_bindgen_test]
fn test_op_9xy0() {
    let mut processor = build_processor();
    processor.execute(0x90e0);
    assert_eq!(processor.pc, SKIPPED_PC);
    let mut processor = build_processor();
    processor.execute(0x9010);
    assert_eq!(processor.pc, NEXT_PC);
}

// LD I, byte
#[wasm_bindgen_test]
fn test_op_annn() {
    let mut processor = build_processor();
    processor.execute(0xa123);
    assert_eq!(processor.i_reg, 0x123);
}

// JP V0, addr
#[wasm_bindgen_test]
fn test_op_bnnn() {
    let mut processor = build_processor();
    processor.test_set_registers(0, 3);
    processor.execute(0xb123);
    assert_eq!(processor.pc, 0x126);
}

// RND Vx, byte
// Generates random u8, then ANDs it with kk.
// We can't test randomness, but we can test the AND.
#[wasm_bindgen_test]
fn test_op_cxkk() {
    let mut processor = build_processor();
    processor.execute(0xc000);
    assert_eq!(processor.test_get_registers(0), 0);
    processor.execute(0xc00f);
    assert_eq!(processor.test_get_registers(0) & 0xf0, 0);
}

// DRW Vx, Vy, nibble
#[wasm_bindgen_test]
fn test_op_dxyn() {
    let mut processor = build_processor();
    processor.i_reg = 0;
    processor.test_set_ram(0, 0b11111111);
    processor.test_set_ram(1, 0b00000000);

    processor.display.set_pixel(0, 0, true);
    processor.display.set_pixel(0, 1, false);
    processor.display.set_pixel(1, 0, true);
    processor.display.set_pixel(1, 1, false);

    processor.test_set_registers(0, 0);
    processor.execute(0xd002);

    assert_eq!(processor.display.get_pixel(0, 0 ), false);
    assert_eq!(processor.display.get_pixel(0, 1 ), true);
    assert_eq!(processor.display.get_pixel(1, 0 ), true);
    assert_eq!(processor.display.get_pixel(1, 1 ), false);
    assert_eq!(processor.test_get_registers(Register::VF as usize), 1);
    assert_eq!(processor.pc, NEXT_PC);
}


#[wasm_bindgen_test]
fn test_op_dxyn_wrap_horizontal() {
    let mut processor = build_processor();

    let x = DISPLAY_WIDTH - 4;

    processor.i_reg = 0;
    processor.test_set_ram(0, 0b11111111);
    processor.test_set_registers(0, x as u8);
    processor.test_set_registers(1, 0);

    processor.execute(0xd011);

    assert_eq!(processor.display.get_pixel(0, x-1 ), false);
    assert_eq!(processor.display.get_pixel(0, x ), true);
    assert_eq!(processor.display.get_pixel(0, x+1 ), true);
    assert_eq!(processor.display.get_pixel(0, x+2 ), true);
    assert_eq!(processor.display.get_pixel(0, x+3 ), true);
    assert_eq!(processor.display.get_pixel(0, 0 ), true);
    assert_eq!(processor.display.get_pixel(0, 1 ), true);
    assert_eq!(processor.display.get_pixel(0, 2 ), true);
    assert_eq!(processor.display.get_pixel(0, 3 ), true);
    assert_eq!(processor.display.get_pixel(0, 4 ), false);

    assert_eq!(processor.test_get_registers(Register::VF as usize), 0);
}

// DRW Vx, Vy, nibble
#[wasm_bindgen_test]
fn test_op_dxyn_wrap_vertical() {
    let mut processor = build_processor();
    let y = DISPLAY_HEIGHT - 1;

    processor.i_reg = 0;
    processor.test_set_ram(0, 0b11111111);
    processor.test_set_ram(1, 0b11111111);

    processor.test_set_registers(0, 0);
    processor.test_set_registers(1, y as u8);

    processor.execute(0xd012);

    assert_eq!(processor.display.get_pixel(y, 0 ), true);
    assert_eq!(processor.display.get_pixel(0, 0 ), true);

    assert_eq!(processor.test_get_registers(Register::VF as usize), 0);
}


// SKP Vx
#[wasm_bindgen_test]
fn test_op_ex9e() {
    let mut processor = build_processor();
    processor.key_press(Key::K0);
    processor.test_set_registers(5, 9);
    processor.execute(0xe59e);
    assert_eq!(processor.pc, SKIPPED_PC);


    let mut processor = build_processor();
    processor.test_set_registers(5, 9);
    processor.execute(0xe59e);
    assert_eq!(processor.pc, NEXT_PC);
}

// LD Vx, DT
#[wasm_bindgen_test]
fn test_op_fx07() {
    let mut processor = build_processor();
    processor.test_set_registers(Register::DT as usize, 20);
    processor.execute(0xf507);
    assert_eq!(processor.test_get_registers(5), 20);
    assert_eq!(processor.pc, NEXT_PC);
}

// LD DT, vX
#[wasm_bindgen_test]
fn test_op_fx15() {
    let mut processor = build_processor();
    processor.test_set_registers(5, 9);
    processor.execute(0xf515);
    assert_eq!(processor.test_get_registers(Register::DT as usize), 9);
    assert_eq!(processor.pc, NEXT_PC);
}

// LD ST, vX
#[wasm_bindgen_test]
fn test_op_fx18() {
    let mut processor = build_processor();
    processor.test_set_registers(5, 9);
    processor.execute(0xf518);
    assert_eq!(processor.test_get_registers(Register::ST as usize), 9);
    assert_eq!(processor.pc, NEXT_PC);
}

// ADD I, Vx
#[wasm_bindgen_test]
fn test_op_fx1e() {
    let mut processor = build_processor();
    processor.test_set_registers(5, 9);
    processor.i_reg = 9;
    processor.execute(0xf51e);
    assert_eq!(processor.i_reg, 18);
    assert_eq!(processor.pc, NEXT_PC);
}

// LD F, Vx
#[wasm_bindgen_test]
fn test_op_fx29() {
    let mut processor = build_processor();
    processor.test_set_registers(5, 9);
    processor.execute(0xf529);
    assert_eq!(processor.i_reg, 5 * 9);
    assert_eq!(processor.pc, NEXT_PC);

}

// LD B, Vx
#[wasm_bindgen_test]
fn test_op_fx33() {
    let mut processor = build_processor();
    processor.test_set_registers(5, 123);
    processor.i_reg = 1000;
    processor.execute(0xf533);
    assert_eq!(processor.test_get_ram(1000), 1);
    assert_eq!(processor.test_get_ram(1001), 2);
    assert_eq!(processor.test_get_ram(1002), 3);
    assert_eq!(processor.pc, NEXT_PC);

}

// LD [I], Vx
#[wasm_bindgen_test]
fn test_op_fx55() {
    let mut processor = build_processor();
    processor.i_reg = 1000;
    processor.execute(0xff55);
    for i in 0..16 {
        assert_eq!(processor.test_get_ram(1000 + i as usize), processor.test_get_registers(i));
    }
    assert_eq!(processor.pc, NEXT_PC);
}

// LD Vx, [I]
#[wasm_bindgen_test]
fn test_op_fx65() {
    let mut processor = build_processor();
    for i in 0..16 as usize {
        processor.test_set_ram(1000 + i, i as u8);
    }
    processor.i_reg = 1000;
    processor.execute(0xff65);

    for i in 0..16 as usize {
        assert_eq!(processor.test_get_registers(i), processor.test_get_ram(1000 + i as usize));
    }
    assert_eq!(processor.pc, NEXT_PC);
}

#[wasm_bindgen_test]
fn test_timers() {
    let mut processor = build_processor();
    processor.test_set_registers(Register::DT as usize, 200);
    processor.test_set_registers(Register::ST as usize, 100);
    processor.tick();
    assert_eq!(processor.test_get_registers(Register::DT as usize), 199);
    assert_eq!(processor.test_get_registers(Register::ST as usize), 99);
}
