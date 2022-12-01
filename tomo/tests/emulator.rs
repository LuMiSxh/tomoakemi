use wasm_bindgen_test::*;

use tomo::chip8::DISPLAY_HEIGHT as HOEHE;
use tomo::chip8::DISPLAY_WIDTH as BREITE;
use tomo::chip8::processor::{Key, Processor as CHIP8, Register};

// TODO: Tests beheben
#[wasm_bindgen_test]
fn test_initialization() {
    let emu = CHIP8::new();
    assert_eq!(emu.pc, 512);
}

#[wasm_bindgen_test]
fn test_fetch() {
    let mut emu = CHIP8::new();
    //Setzen fon test-opcodes
    emu.test_set_ram(0x200, 0xAB);
    emu.test_set_ram(0x201, 0xCD);
    // Lesen des ersten opcodes
    let opcode = emu.fetch();
    assert_eq!(opcode, 0xABCD);
}

#[wasm_bindgen_test]
fn test_load_data() {
    let mut emu = CHIP8::new();
    emu.load(vec![1, 2, 3]);
    assert_eq!(emu.test_get_ram(0x200), 1);
    assert_eq!(emu.test_get_ram(0x201), 2);
    assert_eq!(emu.test_get_ram(0x202), 3);
}


#[wasm_bindgen_test]
fn test_execute_0x1000() {
    let mut emu = CHIP8::new();
    // Test JP-Operation
    emu.execute(0x1FED);
    assert_eq!(emu.pc, 0x0FED);
}

#[wasm_bindgen_test]
fn test_execute_0x2000() {
    let mut emu = CHIP8::new();
    // Test CALL-Operation
    emu.pc = 0xDEAD;
    emu.execute(0x2FED);
    assert_eq!(emu.pc, 0xFEF);
    assert_eq!(emu.test_get_stack(emu.sp as usize), 0xDEAD);
}

#[wasm_bindgen_test]
fn test_execute_0x3000() {
    let mut emu = CHIP8::new();
    // Test SKIP-Operation
    emu.pc = 0;
    emu.test_set_registers(0, 0xAD);
    emu.execute(0x30AD);
    assert_eq!(emu.pc, 4);

    emu.pc = 0;
    emu.test_set_registers(0, 0);
    emu.execute(0x30AD);
    assert_eq!(emu.pc, 2);
}

#[wasm_bindgen_test]
fn test_execute_0x4000() {
    let mut emu = CHIP8::new();
    // Test NE SKIP-Operation
    emu.pc = 0;
    emu.test_set_registers(0, 0xAD);
    emu.execute(0x40AD);
    assert_eq!(emu.pc, 0);

    emu.pc = 0;
    emu.test_set_registers(0, 0);
    emu.execute(0x40AD);
    assert_eq!(emu.pc, 2);
}

#[wasm_bindgen_test]
fn test_execute_0x5000() {
    let mut emu = CHIP8::new();
    emu.pc = 0;
    emu.test_set_registers(0x0, 0xAB);
    emu.test_set_registers(0x1, 0xAB);

    emu.execute(0x5010);
    assert_eq!(emu.pc, 2);

    emu.pc = 0;
    emu.test_set_registers(0x0, 0xAB);
    emu.test_set_registers(0x1, 0xCD);

    emu.execute(0x5010);
    assert_eq!(emu.pc, 0);
}

#[wasm_bindgen_test]
fn test_execute_0x6000() {
    let mut emu = CHIP8::new();
    emu.execute(0x60AB);
    assert_eq!(emu.test_get_registers(0), 0xAB);
}

#[wasm_bindgen_test]
fn test_execute_0x7000() {
    let mut emu = CHIP8::new();
    emu.test_set_registers(0, 2);

    emu.execute(0x7002);
    assert_eq!(emu.test_get_registers(0), 4);
}

#[wasm_bindgen_test]
fn test_execute_0x8000() {
    let mut emu = CHIP8::new();
    // LD
    emu.test_set_registers(1, 0xAD);
    emu.execute(0x8010);
    assert_eq!(emu.test_get_registers(0), 0xAD);
    // OR
    emu.test_set_registers(0, 0xF0);
    emu.test_set_registers(1, 0x0F);

    emu.execute(0x8011);
    assert_eq!(emu.test_get_registers(0), 0xFF);
    // AND
    emu.test_set_registers(0, 0xF0);
    emu.test_set_registers(1, 0x0F);

    emu.execute(0x8012);
    assert_eq!(emu.test_get_registers(0), 0x00);
    // XOR
    emu.test_set_registers(0, 0xF0);
    emu.test_set_registers(1, 0x0F);
    emu.execute(0x8013);
    assert_eq!(emu.test_get_registers(0), 0xFF);
    // ADD
    emu.test_set_registers(0, 0x02);
    emu.test_set_registers(1, 0x02);

    emu.execute(0x8014);
    assert_eq!(emu.test_get_registers(0), 4);
    // SUB
    emu.test_set_registers(0, 0x02);
    emu.test_set_registers(1, 0x02);

    emu.execute(0x8015);
    assert_eq!(emu.test_get_registers(0), 0);
    // SHR
    emu.test_set_registers(0, 0x01);

    emu.execute(0x8006);
    // Rechts-Verschiebung sollte ergeben: VF = 1, V1 = 0
    assert_eq!(emu.test_get_registers(15), 1);
    assert_eq!(emu.test_get_registers(0), 0);
    // Rechts-Verschiebung sollte ergeben: VF = 0, V1 = 1
    emu.test_set_registers(0, 0b0010);

    emu.execute(0x8006);
    assert_eq!(emu.test_get_registers(15), 0);
    assert_eq!(emu.test_get_registers(0), 0b0001);
    // SUBN vx, vy
    emu.test_set_registers(0, 0x02);
    emu.test_set_registers(1, 0x04);

    emu.execute(0x8017);
    assert_eq!(emu.test_get_registers(15), 1);
    assert_eq!(emu.test_get_registers(0), 2);
    // SHL vx
    emu.test_set_registers(0, 0b1000_0000);

    emu.execute(0x800E);
    assert_eq!(emu.test_get_registers(15), 1);
    assert_eq!(emu.test_get_registers(0), 0);

    emu.test_set_registers(0, 0b0000_0001);

    emu.execute(0x800E);
    assert_eq!(emu.test_get_registers(15), 0);
    assert_eq!(emu.test_get_registers(0), 0b0010);
}

#[wasm_bindgen_test]
fn test_execute_0x9000() {
    let mut emu = CHIP8::new();
    emu.pc = 0;
    emu.test_set_registers(0, 0xAB);
    emu.test_set_registers(1, 0xCD);

    emu.execute(0x9010);
    assert_eq!(emu.pc, 4);
}

#[wasm_bindgen_test]
fn test_execute_0xa000() {
    let mut emu = CHIP8::new();
    emu.execute(0xABCD);
    assert_eq!(emu.i_reg, 0xBCD);
}

#[wasm_bindgen_test]
fn test_execute_0xb000() {
    let mut emu = CHIP8::new();

    emu.test_set_registers(0, 0xF);

    emu.execute(0xBCD0);
    assert_eq!(emu.pc, 0xCDF);
}

#[wasm_bindgen_test]
fn test_execute_0xc000() {
    let mut emu = CHIP8::new();
    emu.execute(0xC0AD);
    assert_ne!(emu.test_get_registers(0), 0);
}

#[wasm_bindgen_test]
fn test_execute_0xd000() {
    let mut emu = CHIP8::new();
    // Fake sprite.
    emu.test_set_ram(0, 0xFF);

    emu.execute(0xD001);
    // VF-Register sollte 0 sein
    assert_eq!(emu.test_get_registers(Register::VF as usize), 0);
    // Test, ob der Sprite in den Display-Speicher gelesen wurde
    for idx in 0..8 {
        assert_eq!(emu.display.get_pixel(0, idx), true);
    }
    // Sprite an die selbe Stelle zu schreiben, sollte das VF-Register zurücksetzen
    emu.test_set_ram(0, 0xFF);
    emu.execute(0xD001);
    assert_eq!(emu.test_get_registers(Register::VF as usize), 1);
    // Test, ob der Sprite in den Display-Speicher gelesen wurde
    for idx in 0..8 {
        assert_eq!(emu.display.get_pixel(0, idx), false);
    }

    // Test horizontales wrapping
    emu.test_set_ram(0, 0xFF);
    emu.test_set_ram(1, 0xFF);

    emu.test_set_registers(0, (BREITE - 1) as u8);
    emu.test_set_registers(1, 0);

    emu.execute(0xD011);
    // Sollte rechts starten und links weitergehen
    assert_eq!(emu.display.get_pixel(0, BREITE - 1), true);
    emu.display.set_pixel(0, BREITE - 1, false);

    for idx in 0..7 {
        assert_eq!(emu.display.get_pixel(0, idx), true);
        // Zurücksetzen auf 0
        emu.display.set_pixel(0, idx, false);
    }

    emu.test_set_registers(0, (BREITE - 1) as u8);
    emu.test_set_registers(1, (HOEHE - 1) as u8);

    emu.execute(0xD012);
    // Rechts oben und Links unten setzen
    assert_eq!(emu.display.get_pixel(0, BREITE - 1), true);
    assert_eq!(emu.display.get_pixel(HOEHE - 1, 0), true);
    // Links oben 7 und unten links 7 pixel
    for idx in 0..7 {
        assert_eq!(emu.display.get_pixel(0, idx), true);
        assert_eq!(emu.display.get_pixel(HOEHE - 1, idx), true);
    }
}

#[wasm_bindgen_test]
fn test_execute_0xf000() {
    let mut emu = CHIP8::new();
    emu.test_set_registers(0, 123);

    emu.execute(0xF033);
    // Sollte die Zahlen 1, 2 und 3 in jeweils einem individuellen Speicherplatz haben
    for idx in 0..3 {
        assert_eq!(emu.test_get_ram(idx), (idx + 1) as u8);
    }

    // Simulation Knopfdruck
    let old_pc = emu.pc;
    emu.execute(0xF00A);
    // PC sollte um 2 kleiner werden um warten zu simulieren, während auf die Eingabe gewartet wird
    assert_eq!(old_pc - 2, emu.pc);
    emu.current_key = Some(Key::KA);
    emu.execute(0xF00A);
    assert_eq!(emu.test_get_registers(0), 0x0A);
}
