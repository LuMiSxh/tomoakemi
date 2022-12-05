use crate::{err, log};
use crate::chip8::display::Display;
use crate::prelude::*;

use super::{DISPLAY_HEIGHT, DISPLAY_WIDTH, FONT, OPCODE_SIZE, RAM_SIZE, REGISTER_SIZE, STACK_SIZE};

// Zähler, der nach jedem Fetch bestimmt, worauf der PC gestellt werden muss
#[derive(Debug)]
enum ProgramCounter {
    Next,
    Skip,
    Block,
    Jump(usize),
}

impl ProgramCounter {
    fn skip_if(condition: bool) -> ProgramCounter {
        if condition {
            ProgramCounter::Skip
        } else {
            ProgramCounter::Next
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct Output {
    pub success: bool,
    pub opcode: u16,
}

// Tasten als Klassen-Repräsentation
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Key {
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
    K0,
    KA,
    KB,
    KC,
    KD,
    KE,
    KF,
}

// Register als Klassen-Repräsentation
#[wasm_bindgen]
#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Register {
    // Standard Register außer VF - Wird als Flaggenträger genutzt
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
    // DT = Delay Timer = Verzögerung's-Timer
    DT,
    // ST = Sound Timer = Geräusche-Timer
    ST,
}

#[wasm_bindgen]
pub struct Processor {
    // RAM / Speicher des CHIP8. Besteht aus 4kb
    ram: [u8; RAM_SIZE],
    // "Program Counter" (Programmzähler) Pointer der
    // auf die aktuelle Instruktion im Speicher zeigt
    pub pc: u16,
    // "Index register" Pointer der auf Stellen im Speicher zeigt
    pub i_reg: u16,
    // "Stack" 16-bit Adressen für Funktionen, auf die der CPU
    // zurückgreifen kann um Rückgaben zu erhalten
    stack: [u16; STACK_SIZE],
    // "Stack pointer" Pointer der auf Stellen im Stack
    pub sp: u16,
    // Register, um alles mögliche des Programmes (Wie Variablen) zu speichern
    // Register VF wird aber hauptsächlich als Flaggenträger genutzt und
    // wird daher nur auf "0" oder "1" gestellt
    // Register:
    // V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, VA, VB, VC, VD, VE, VF
    registers: [u8; REGISTER_SIZE],
    // Display des Emulators
    pub display: Display,
    // Speicher für gedrückte Tasten
    keys: [bool; 16],
    // Speichert die zuletzt gedrückte Taste
    pub current_key: Option<Key>,
}

#[wasm_bindgen]
impl Processor {
    // Konstruktor
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Error Hook-Initialisieren
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        let mut proc = Processor {
            ram: [0; RAM_SIZE],
            // PC muss auf den Hex-Wert von 512 gesetzt werden,
            // da alle Werte darunter im Speicher ursprünglich
            // für den Interpreter genutzt wurden
            // (Entfällt im Emulator aber)
            pc: 0x200,
            i_reg: 0,
            stack: [0; STACK_SIZE],
            sp: 0,
            registers: [0; REGISTER_SIZE],
            display: Display::new(),
            keys: [false; 16],
            current_key: None,
        };

        proc.initialize();
        proc
    }

    // Opcode aus dem RAM holen
    pub fn fetch(&mut self) -> u16 {
        let pc = self.pc as usize;
        u16::from(self.ram[pc]) << 8 | u16::from(self.ram[pc + 1])
    }

    // Ausführen des Opcodes
    pub fn execute(&mut self, opcode: u16) -> Output {
        // Opcode auftrennen in verschiedene Nibbles, Register und Instruktionen
        let nibbles = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );
        let nnn = (opcode & 0x0FFF) as usize;
        let kk = (opcode & 0x00FF) as u8;
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;

        let mut success: bool = true;

        let pc_change: ProgramCounter = match nibbles {
            (0x00, 0x00, 0x0e, 0x00) => {
                // CLS: Display leeren
                self.display.cls();
                ProgramCounter::Next
            }
            (0x00, 0x00, 0x0e, 0x0e) => {
                // RET: Rückgabe einer Subroutine
                self.sp -= 1;
                ProgramCounter::Jump(self.stack[self.sp as usize] as usize)
            }
            (0x01, _, _, _) => {
                // JP <addr>: Springen zur gegebenen Adresse
                ProgramCounter::Jump(nnn)
            }
            (0x02, _, _, _) => {
                // CALL <addr>: Ruft die Subroutine an gegebener
                // Adresse auf
                self.stack[self.sp as usize] = (self.pc as usize + OPCODE_SIZE) as u16;
                self.sp += 1;
                ProgramCounter::Jump(nnn)
            }
            (0x03, _, _, _) => {
                // SE (Vx, Kk): Überspringen der nächsten Instruktion,
                // wenn Vx == Kk
                ProgramCounter::skip_if(self.registers[x] == kk)
            }
            (0x04, _, _, _) => {
                // SNE (Vx, Kk): Überspringen der nächsten Instruktion,
                // wenn Vx != Kk
                ProgramCounter::skip_if(self.registers[x] != kk)
            }
            (0x05, _, _, 0x00) => {
                // SE (Vx, Vy): Überspringen der nächsten Instruktion,
                // wenn Vx == Vy
                ProgramCounter::skip_if(self.registers[x] == self.registers[y])
            }
            (0x06, _, _, _) => {
                // LD (Vx, Kk): Setzt das Register von Vx auf  Kk-Bytes
                self.registers[x] = kk;
                ProgramCounter::Next
            }
            (0x07, _, _, _) => {
                // ADD (Vx, Kk): Addiert Kk auf den Wert des Registers
                // Vx und speicher dies dort
                let vx = self.registers[x] as u16;
                let val = kk as u16;
                let result = vx + val;
                self.registers[x] = result as u8;
                ProgramCounter::Next
            }
            (0x08, _, _, 0x00) => {
                // LD (Vx, Vy): Setzen des Registers von Vx auf
                // den Wer des Registers von Vy
                self.registers[x] = self.registers[y];
                ProgramCounter::Next
            }
            (0x08, _, _, 0x01) => {
                // OR (Vx, Vy): Bit-OR Operation zwischen Register
                // Vx und Vy mit speicherung des Wertes in Vx
                self.registers[x] |= self.registers[y];
                ProgramCounter::Next
            }
            (0x08, _, _, 0x02) => {
                // AND (Vx, Vy): Bit-AND Operation zwischen Register
                // Vx und Vy mit speicherung des Wertes in Vx
                self.registers[x] &= self.registers[y];
                ProgramCounter::Next
            }
            (0x08, _, _, 0x03) => {
                // XOR (Vx, Vy): Bit-XOR Operation zwischen Register
                // Vx und Vy mit speicherung des Wertes in Vx
                self.registers[x] ^= self.registers[y];
                ProgramCounter::Next
            }
            (0x08, _, _, 0x04) => {
                // ADD (Vx, Vy): Addieren des Register Wertes von Vy
                // und Vx und speicherung des Wertes in Vx
                // Wenn Vx größer als u8 ist, wird VF auf 1 gesetzt,
                // andernfalls auf 0 und es wird nur die u8 Form der
                // Zahl gespeichert
                let vx = self.registers[x] as u16;
                let vy = self.registers[y] as u16;
                let result = vx + vy;
                self.registers[x] = result as u8;
                self.registers[Register::VF as usize] = if result > 0xFF { 1 } else { 0 };
                ProgramCounter::Next
            }
            (0x08, _, _, 0x05) => {
                // SUB (Vx, Vy): Subtrahieren des Register Wertes von Vy
                // und Vx und speicherung des Wertes in Vx
                // Wenn Vy > Vx, dann VF 1, andernfalls 0
                self.registers[Register::VF as usize] = if self.registers[x] > self.registers[y] { 1 } else { 0 };
                self.registers[x] = self.registers[x].wrapping_sub(self.registers[y]);
                ProgramCounter::Next
            }
            (0x08, _, _, 0x06) => {
                // SHR (Vx): Wenn das unbedeutendste Bit von VX 1 ist, wird VF auf 1
                // gesetzt, ansonsten 0 und Vx wird durch 2 geteilt
                self.registers[Register::VF as usize] = self.registers[x] & 1;
                self.registers[x] >>= 1;
                ProgramCounter::Next
            }
            (0x08, _, _, 0x07) => {
                // SUBN (Vx, Vy): Vx wird zu Vy minus Vx
                // Wenn Vy > Vx, dann VF 1, andernfalls 0
                self.registers[Register::VF as usize] = if self.registers[y] > self.registers[x] { 1 } else { 0 };
                self.registers[x] = self.registers[y].wrapping_sub(self.registers[x]);
                ProgramCounter::Next
            }
            (0x08, _, _, 0x0e) => {
                // SHL (Vx): If das wichtigste Bit von Vx 1 ist, wird VF auf 1 gesetzt,NB
                // ansonsten 0 und Vx wird um 2 multipliziert
                let msb = (self.registers[x] & 0b1000_0000) >> 7;
                self.registers[Register::VF as usize] = msb;
                self.registers[x] <<= 1;
                ProgramCounter::Next
            }
            (0x09, _, _, 0x00) => {
                // SNE (Vx, Vy): Überspringen der nächsten Instruktion,
                // wenn Vx != Vy
                ProgramCounter::skip_if(self.registers[x] != self.registers[y])
            }
            (0x0a, _, _, _) => {
                // LD (I_reg) <addr>: Verschiebt das Index-Register auf
                // die gegebene Adresse
                self.i_reg = nnn as u16;
                ProgramCounter::Next
            }
            (0x0b, _, _, _) => {
                // JP (V0) <addr>: Spingt zur Adresse (V0 + Adresse)
                ProgramCounter::Jump(nnn + self.registers[Register::V0 as usize] as usize)
            }
            (0x0c, _, _, _) => {
                // RND (Vx, Kk): Generiert eine zufällige Zahl zwischen 0 und 255
                // Welche über den Bit-AND Operator mit Kk verschmolzen wird und
                // im Register von Vx gespeichert wird
                self.registers[x] = random_byte() & kk;
                ProgramCounter::Next
            }
            (0x0d, _, _, _) => {
                // DRW (Vx, Vy, n): Liest n-Bytes aus dem RAM mit dem Startpunkt im
                // Index-Register. DIe Bytes werden dann als "Sprite" auf dem Bildschirm
                // an der Stelle (Vx | Vy) dargestellt. Wenn an der Stelle ein Pixel
                // gelöscht wird, wird das VF Register auf 1 gestellt, ansonsten 0
                self.registers[Register::VF as usize] = 0;
                for byte in 0..n {
                    let y = (self.registers[y] as usize + byte) % DISPLAY_HEIGHT;
                    for bit in 0..8 {
                        let x = (self.registers[x] as usize + bit) % DISPLAY_WIDTH;
                        let color = (self.ram[self.i_reg as usize + byte] >> (7 - bit)) & 1;
                        self.registers[Register::VF as usize] |= u8::from(color & (if self.display.get_pixel(y, x) { 1 } else { 0 }));
                        self.display.set_pixel(y, x, ((if self.display.get_pixel(y, x) { 1 } else { 0 }) ^ color) == 1);
                    }
                }

                ProgramCounter::Next
            }
            (0x0e, _, 0x09, 0x0e) => {
                // SKP (Vx): Überspringt die nächste Instruktion, wenn die
                // korrespondierende Taste gedrückt ist
                let key = self.registers[x];
                ProgramCounter::skip_if(self.keys[key as usize])
            }
            (0x0e, _, 0x0a, 0x01) => {
                // SKNP (Vx): Überspringt nächste Instruktion, wenn die
                // korrespondierende Taste nicht gedrückt ist
                let key = self.registers[x];
                ProgramCounter::skip_if(!self.keys[key as usize])
            }
            (0x0f, _, 0x00, 0x07) => {
                // LD (Vx, DT): Setzt das Vx-Register zu dem Wert des Delay-Timers
                self.registers[x] = self.registers[Register::DT as usize];
                ProgramCounter::Next
            }
            (0x0f, _, 0x00, 0x0a) => {
                // LD (Vx, K): Wenn kein Knopf gedrückt wurde, "blockiert" der CPU
                // bis die richtige Taste gedrückt wurde, indem wir 2 Opcodes zurückspringen
                // und wieder hier landen
                let res = match self.current_key {
                    Some(key) => {
                        self.registers[x] = key as u8;
                        ProgramCounter::Next
                    }
                    None => ProgramCounter::Block
                };
                res
            }
            (0x0f, _, 0x01, 0x05) => {
                // LD (DTm Vx): Setzt den Delay-Timer auf den Wert des Registers von Vx
                self.registers[Register::DT as usize] = self.registers[x];
                ProgramCounter::Next
            }
            (0x0f, _, 0x01, 0x08) => {
                // LD (ST, Vx): Setzt den Sound-Timer auf den Wert des Registers von Vx
                self.registers[Register::ST as usize] = self.registers[x];
                ProgramCounter::Next
            }
            (0x0f, _, 0x01, 0x0e) => {
                // ADD (I_reg, Vx): Index Register wird um den Wer des Vx Registers erhöht
                self.i_reg += u16::from(self.registers[x]);
                self.registers[Register::VF as usize] = if self.i_reg > 0x0F00 {1} else {0};
                ProgramCounter::Next
            }
            (0x0f, _, 0x02, 0x09) => {
                // LD (F, Vx): Index Register wird auf den Hex Wer (Darum *5) für die Position
                // eines Sprite aus dem Wert des Vx Registers gestellt
                self.i_reg = ( self.registers[x] * 5) as u16;
                ProgramCounter::Next
            }
            (0x0f, _, 0x03, 0x03) => {
                // LD (B, Vx): Nimmt den Dezimalwert von Vx und Platziert
                // - Hunderterstelle im RAM an Stelle Index Register
                // - Zehnerstelle im RAM an stelle Index Register +1
                // - Einerstelle im RAM an Stelle Index Register +2
                self.ram[self.i_reg as usize] = self.registers[x] / 100;
                self.ram[(self.i_reg + 1) as usize] = (self.registers[x] % 100) / 10;
                self.ram[(self.i_reg + 2) as usize] = self.registers[x] % 10;
                ProgramCounter::Next
            }
            (0x0f, _, 0x05, 0x05) => {
                // LD (I, Vx): Kopiert alle Register in den RAM mit Startpunkt im Index Register
                for i in 0..REGISTER_SIZE {
                    self.ram[(self.i_reg as usize + i) as usize] = self.registers[i];
                }
                ProgramCounter::Next
            }
            (0x0f, _, 0x06, 0x05) => {
                // LD (Vx, I): Liest alle Werte aus dem RAM mit Startpunkt im Index Register
                // und kopiert diese in alle Register korrespondierend
                for i in 0..REGISTER_SIZE {
                    self.registers[i] = self.ram[self.i_reg as usize + i];
                }
                ProgramCounter::Next
            }
            _ => {
                err!("The provided Opcode (`{:#X}`) is not supported or invalid", opcode);
                success = false;
                ProgramCounter::Next
            }
        };

        match pc_change {
            ProgramCounter::Next => self.pc += OPCODE_SIZE as u16,
            ProgramCounter::Skip => self.pc += 2 * OPCODE_SIZE as u16,
            ProgramCounter::Block => self.pc -= OPCODE_SIZE as u16,
            ProgramCounter::Jump(addr) => self.pc = addr as u16,
        }

        Output {
            success,
            opcode
        }
    }


    // Wird genutzt, um die Schrift in den RAM zu laden
    fn initialize(&mut self) {
        for i in 0..80 {
            self.ram[i] = FONT[i];
        }
    }

    pub fn tick(&mut self) -> Output {
        if self.registers[Register::DT as usize] > 0 {
            self.registers[Register::DT as usize] -= 1;
        }

        if self.registers[Register::ST as usize] > 0 {
            self.registers[Register::ST as usize] -= 1;
        }

        let opcode = self.fetch();
        self.execute(opcode)
    }

    // Boolean, ob ein Piep-Ton gespielt werden soll
    pub fn should_beep(&self) -> bool {
        self.registers[Register::ST as usize] > 0
    }

    // Taste gedrückt
    pub fn key_press(&mut self, key: Key) {
        self.current_key = Some(key);
        self.keys[key as usize] = true;
    }

    // Taste losgelassen
    pub fn key_up(&mut self, key: Key) {
        if let Some(current_key) = self.current_key {
            if key == current_key {
                self.current_key = None;
            }
        }
        self.keys[key as usize] = false;
    }

    pub fn reset(&mut self) {
        // Speicher leeren
        for i in 0..RAM_SIZE {
            self.ram[i] = 0;
        }

        // Stack leeren
        for i in 0..STACK_SIZE {
            self.stack[i] = 0;
        }

        // Register leeren
        for i in 0..REGISTER_SIZE {
            self.registers[i] = 0;
        }
        self.sp = 0;
        self.pc = 0x200;

        // Display leeren
        self.display.cls();

        // Schrift neu hinzufügen
        self.initialize();
    }

    // Laden von Daten in den CPU
    pub fn load(&mut self, data: Vec<u8>) -> usize {
        self.reset();
        let mut start = 0;
        self.pc = 0x200;
        for byte in data.iter() {
            self.ram[self.pc as usize + start] = *byte;
            start += 1;
        }
        log!("Tomo: Data was successfully loaded into the ram with a size of {} bytes.", start);
        start
    }

    pub fn test_set_registers(&mut self, idx: usize, data: u8) {
        self.registers[idx] = data;
    }

    pub fn test_get_registers(&mut self, idx: usize) -> u8 {
        self.registers[idx]
    }

    pub fn test_set_ram(&mut self, idx: usize, data: u8) {
        self.ram[idx] = data;
    }

    pub fn test_get_ram(&mut self, idx: usize) -> u8 {
        self.ram[idx]
    }

    pub fn test_get_stack(&mut self, idx: usize) -> u16 {
        self.stack[idx]
    }

    pub fn test_set_stack(&mut self, idx: usize, data: u16) {
        self.stack[idx] = data;
    }
}
