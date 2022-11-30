use crate::chip8::display::{BREITE, Display, FONT, HOEHE};
use crate::log;
use crate::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct Output {
    pub success: bool,
    pub edited_pixels: Vec<usize>,
}

// Die Struktur stellt das Gerüst einer Klasse dar. In diesem Fall den CPU
#[wasm_bindgen]
pub struct Cpu {
    // "Memory" (Speicher) bestehend aus 4kb
    mem: [u8; 4096],
    // "Program Counter" (Programmzähler) Pointer der auf die aktuelle Instruktion im Speicher zeigt
    pub pc: u16,
    // "Index register" Pointer der auf Stellen im Speicher zeigt
    pub i: u16,
    // "Stack" 16-bit Adressen für Funktionen, auf die der CPU zurückgreifen kann um Rückgaben zu erhalten
    stack: [u16; 16],
    // "Stack pointer" Pointer der auf Stellen im Stack
    pub sp: u16,
    // "Delay Timer" 8-bit Zahl, die 60 mal pro Sekunde verringert wird, bis er null erreicht
    pub dt: u8,
    // "Sound Timer" 8-bit Zahl, die 60 mal pro Sekunde verringert wird, bis er null erreicht
    // Ist der ST > 0 wird ein Ton abgespielt
    pub st: u8,
    // Register, um alles mögliche des Programmes (Wie Variablen) zu speichern
    // Register VF wird aber hauptsächlich als Flaggenträger genutzt und wird daher nur auf "0" oder "1" gestellt
    // Register:
    // V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, VA, VB, VC, VD, VE, VF
    registers: [u8; 16],
    // Display des Emulators
    pub display: Display,
    // Speicher für gedrückte Tasten
    keys: [bool; 16],
    // Speichert die zuletzt gedrückte Taste
    pub current_key: Option<u8>,
}

// Implementation für die Klasse und seine Methoden
#[wasm_bindgen]
impl Cpu {
    // Konstruktor
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Error Hook-Initialisieren
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
        let mut cpu = Cpu {
            mem: [0; 4096],
            // PC muss auf den Hex-Wert von 512 gesetzt werden, da alle Werte darunter im Speicher
            // ursprünglich für den Interpreter genutzt wurden (Entfällt im Emulator aber)
            pc: 0x200,
            i: 0,
            stack: [0; 16],
            sp: 0,
            dt: 0,
            st: 0,
            registers: [0; 16],
            display: Display::new(),
            keys: [false; 16],
            current_key: None,
        };

        // Laden der Schrift
        cpu.set_font();

        cpu
    }

    // Gibt true oder false zurück, je nach dem ob der Sound-Timer positiv ist oder nicht
    pub fn should_beep(&self) -> bool {
        self.st > 0
    }

    // Fetch gibt den nächsten Opcode im Speicher zurück
    pub fn fetch(&mut self) -> u16 {
        let pc = self.pc as usize;
        log!("PC: {}, PC+1: {}", pc, pc+1);
        // Verschieben der ersten 8 bytes nach links und eine OR-Operation an den hinteren 8
        let opcode = u16::from(self.mem[pc]) << 8 | u16::from(self.mem[pc + 1]);
        self.pc += 2;
        opcode
    }

    // Ausführen des gegeben Opcodes
    pub fn execute(&mut self, opcode: u16) -> Output {
        // Speicherplatz der modifizierten pixel
        let mut edited_pixels: Vec<usize> = vec![];
        // Auftrennen des Opcodes in die verschiedenen "Nibbles" über den AND-Operator
        let instr = opcode & 0xF000;
        let subinstr = opcode & 0x000F;
        let addr = opcode & 0x0FFF;
        let lower = (opcode & 0x00FF) as u8;

        // Register Positionen
        let vx = ((opcode & 0x0F00) >> 8) as usize;
        let vy = ((opcode & 0x00F0) >> 4) as usize;

        // Herausarbeiten, welche Instruktion gefragt ist und den dementsprechenden Code ausführen
        match instr {
            0x0000 => {
                match lower {
                    0xE0 => {
                        // Display leeren
                        self.display.clear_display();
                        edited_pixels = (0..(HOEHE * BREITE)).collect();
                    }
                    0xEE => {
                        // Rückgabe von einer Subroutine
                        self.pc = self.stack[self.sp as usize];
                        self.sp -= 1;
                    }
                    _ => {
                        log!("Opcode is invalid: {:#X}", opcode);
                        log!("instr: {:#X}, subinstr: {:#X}, lower: {:#X}", instr, subinstr, lower);
                        log!("Hier?");
                        return Output {
                            success: false,
                            edited_pixels,
                        };
                    }
                }
            }
            0x1000 => {
                // JP <adresse>: Springe va pc zur nächsten Adresse
                self.pc = addr as u16;
            }
            0x2000 => {
                // CALL <adresse>: Rufe die Subroutine bei gegebener Adresse auf
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = addr as u16;
            }
            0x3000 => {
                // SNE Vx, Byte: Überspring nächste Instruktion, wenn VX == byte (Var: lower)
                if self.registers[vx] == lower {
                    self.pc += 2;
                }
            }
            0x4000 => {
                // SNE Vx, Byte: Überspringt nächste Instruktion, wenn VX != byte (Var: lower)
                if self.registers[vx] != lower {
                    self.pc += 2;
                }
            }
            0x5000 => {
                // SNE Vx, Vy: Überspringt nächste Instruktion, wenn vx == vy
                if self.registers[vx] == self.registers[vy] {
                    self.pc += 2;
                }
            }
            0x6000 => {
                // LD Vx, Byte: Setzt das Register von Vx auf die unteren Bytes
                self.registers[vx] += lower;
            }
            0x7000 => {
                // ADD Vx, Byte: Addiert die unteren Bytes zu dem Register von Vx
                self.registers[vx] += lower;
            }
            0x8000 => {
                match subinstr {
                    0 => {
                        // LD Vx, Vy
                        self.registers[vx] = self.registers[vy];
                    }
                    1 => {
                        // OR Vx, Vy
                        self.registers[vx] |= self.registers[vy];
                    }
                    2 => {
                        // AND Vx, Vy
                        self.registers[vx] &= self.registers[vy];
                    }
                    3 => {
                        // XOR Vx, Vy
                        self.registers[vx] ^= self.registers[vy];
                    }
                    4 => {
                        // ADD Vx, Vy
                        self.registers[vx] += self.registers[vy];
                    }
                    5 => {
                        // SUB Vx, Vy
                        self.registers[vx] -= self.registers[vy];
                    }
                    6 => {
                        // SHR Vx {, Vy}
                        let lsb = self.registers[vx] & 1;
                        // 15 für VF
                        self.registers[15] = lsb;
                        self.registers[vx] >>= 1
                    }
                    7 => {
                        // SUBN vx, vy
                        // vx = vy - vx, setze vf = 1 wenn vy > vx
                        if self.registers[vy] > self.registers[vx] {
                            self.registers[15] = 1;
                        } else {
                            self.registers[15] = 0;
                        }
                        self.registers[vx] = self.registers[vy] - self.registers[vx];
                    }
                    0xE => {
                        let msb = (self.registers[vx] & 0b1000_0000) >> 7;
                        // Setze VF-Register zum wichtigsten bit vor der shift-Operation
                        self.registers[15] = msb;
                        self.registers[vx] <<= 1;
                    }
                    _ => {
                        log!("Opcode is invalid: {:#X}", opcode);
                        log!("instr: {:#X}, subinstr: {:#X}, lower: {:#X}", instr, subinstr, lower);
                        return Output {
                            success: false,
                            edited_pixels,
                        };
                    }
                }
            }
            0x9000 => {
                // SNE Vx, Vy: Überspringt nächste Instruktion, wenn vx != vy
                if self.registers[vx] != self.registers[vy] {
                    self.pc += 2;
                }
            }
            0xA000 => {
                // LD i, <adresse>: Verschiebt das Index-Register auf die gegebene Adresse
                self.i = addr;
            }
            0xB000 => {
                // JP V0, <adresse>: Springt via pc zur Adresse V0 + <adresse>
                // 0 für V0
                self.pc = u16::from(self.registers[0]) + addr;
            }
            0xC000 => {
                // RND Vx, byte: Generiert eine zufällige Zahl zwischen 0 und 255, welche via
                // AND-Operator mit den unteren Bytes kombiniert wird und im Register von Vx
                // gespeichert wird
                self.registers[vx] = random_byte() & lower;
            }
            0xD000 => {
                // 15 für VF
                self.registers[15] = 0;
                // Startpunkte für das Sprite
                let mut px = self.registers[vx];
                let mut py = self.registers[vy];

                // Loop durch jede Reihe im Sprite
                for idx in 0..subinstr {
                    let byte = self.mem[(self.i as usize + idx as usize) as usize];
                    for bit_idx in 0..8 {
                        let value = (byte & (0b1000_0000 >> bit_idx)) >> (7 - bit_idx);

                        // Horizontales "Wrapping"
                        let mut wx = px;
                        if px >= (BREITE as u8) {
                            wx = px % BREITE as u8;
                        }

                        // Vertikales "Wrapping"
                        let mut wy = py;
                        if py >= (HOEHE as u8) {
                            wy = py % HOEHE as u8;
                        }

                        let display_idx = (wy as usize) * BREITE + (wx as usize);
                        if display_idx > (HOEHE * BREITE) {
                            log!("wx: {}, wy: {}, px: {}, py: {}", wy, wx, px, py);
                            log!("instr: {:#X}, subinstr: {:#X}, lower: {:#X}", instr, subinstr, lower);
                            log!("Opcode is invalid: {:#X}", opcode);
                            return Output {
                                success: false,
                                edited_pixels,
                            };
                        }

                        // VF setzen, wenn wir einen Pixel löschen
                        if self.registers[15] == 0
                            && value == 1
                            && self.display.get_pixel_state(display_idx)
                        {
                            self.registers[15] = 1;
                        }

                        let mut old_status = if self.display.get_pixel_state(display_idx) { 1 } else { 0 };

                        // XOR für den Display-Wert
                        old_status ^= value;

                        // pixel setzen
                        self.display.set_pixel_state(display_idx, old_status == 1);
                        edited_pixels.push(display_idx);

                        px += 1;
                    }
                    px = self.registers[vx];
                    py += 1;
                }
            }
            0xE000 => {
                match lower {
                    0x9E => {
                        // SKP Vx: Überspringt nächste Instruktion, wenn die korrespondierende
                        // Taste gedrückt ist
                        let key = self.registers[vx as usize];
                        if self.keys[key as usize] {
                            self.pc += 2;
                        }
                    }
                    0xA1 => {
                        // SKNP Vx: Überspringt nächste Instruktion, wenn die korrespondierende
                        // Taste nicht gedrückt ist
                        let key = self.registers[vx as usize];
                        if !self.keys[key as usize] {
                            self.pc += 2;
                        }
                    }
                    _ => {
                        log!("instr: {:#X}, subinstr: {:#X}, lower: {:#X}", instr, subinstr, lower);
                        log!("Opcode is invalid: {:#X}", opcode);
                        return Output {
                            success: false,
                            edited_pixels,
                        };
                    }
                }
            }
            0xF000 => {
                match lower {
                    0x07 => {
                        // LD vx, DT: Setzt das Vx-Register zu dem Wert des DT-Registers (Delay-Timer)
                        self.registers[vx] = self.dt;
                    }
                    0x0A => {
                        // LD vx, k: Wenn kein Knopf gedrückt wurde, "blockieren" wir das Programm,
                        // bis ein Knopf gedrückt wurde, indem wir wieder 2 Instruktionen zurückgehen
                        // Ansonsten: Setze den u8 der Taste auf das Register von Vx
                        match self.current_key {
                            Some(key) => self.registers[vx] = key as u8,
                            None => self.pc -= 2
                        }
                    }
                    0x15 => {
                        // LD dt, vx: Setzt das DT-Register (Delay-Timer) auf den Wert des Vx-Registers
                        // 16 für DT
                        self.dt = self.registers[vx]
                    }
                    0x18 => {
                        //LD St, Vx: Setzt das ST-Register (Sound-Timer) auf den Wert des Vx-Registers
                        self.st = self.registers[vx]
                    }
                    0x1E => {
                        // ADD I, Vx: Index-Register wird um den Wert des Vx-Registers erhöht
                        self.i += u16::from(self.registers[vx])
                    }
                    0x29 => {
                        // LD f, Vx: Index-Register wird auf den Hex-Wert (Darum *5) für die
                        // Position des Sprites aus dem Wert des Vx-Registers gestellt
                        self.i = (vx * 5) as u16
                    }
                    0x33 => {
                        // LD f, Vx: Speicher die Binär-Representation des Wertes des Vx-Registers
                        // im Speicher und den nachfolgenden 2-Indexen dieses
                        let mut num = self.registers[vx];
                        for idx in (0..3).rev() {
                            self.mem[(self.i + idx) as usize] = (num % 10) as u8;
                            num = num / 10;
                        }
                    }
                    0x55 => {
                        // LD [I], Vx: Kopiert die Werte der Register V0 bis VX in den Speicher
                        for idx in 0..vx {
                            self.mem[self.i as usize + idx] = self.registers[idx];
                        }
                    }
                    0x65 => {
                        // LD Vx, [I]
                        for idx in 0..vx {
                            self.registers[idx] = self.mem[self.i as usize + idx];
                        }
                    }
                    _ => {
                        log!("instr: {:#X}, subinstr: {:#X}, lower: {:#X}", instr, subinstr, lower);
                        log!("Opcode is invalid: {:#X}", opcode);
                        return Output {
                            success: false,
                            edited_pixels,
                        };
                    }
                }
            }
            _ => {
                log!("instr: {:#X}, subinstr: {:#X}, lower: {:#X}", instr, subinstr, lower);
                log!("Opcode is invalid: {:#X}", opcode);
                return Output {
                    success: false,
                    edited_pixels,
                };
            }
        }

        Output {
            success: true,
            edited_pixels,
        }
    }

    pub fn reset(&mut self) {
        // Speicher leeren
        for i in 0..4096 {
            self.mem[i] = 0;
        }

        // Stack leeren
        for i in 0..16 {
            self.stack[i] = 0;
        }

        // Register leeren
        for i in 0..16 {
            self.registers[i] = 0;
        }
        self.sp = 0;
        self.pc = 0;

        // Display leeren
        self.display.clear_display();

        // Schrift neu hinzufügen
        self.set_font();
    }

    // Funktion um die Schrift in den Speicher zu laden, damit der CPU Zugriff darauf hat
    fn set_font(&mut self) {
        for i in 0..80 {
            self.mem[i] = FONT[i];
        }
    }

    pub fn pc(&self) -> u16 { self.pc }

    pub fn sp(&self) -> u16 { self.sp }

    // Taste gedrückt
    pub fn key_press(&mut self, key: u8) {
        self.current_key = Some(key);
        self.keys[key as usize] = true;
    }

    // Taste losgelassen
    pub fn key_up(&mut self, key: u8) {
        if let Some(current_key) = self.current_key {
            if key == current_key {
                self.current_key = None;
            }
        }
        self.keys[key as usize] = false;
    }

    // Laden einer ROM (Liste von Bytes)
    pub fn load_rom(&mut self, rom: Option<Box<[u8]>>) -> usize {
        self.reset();

        let mut start = 0;
        self.pc = 0x200;
        if let Some(data) = rom {
            for byte in data.iter() {
                self.mem[self.pc as usize + start] = *byte;
                start += 1;
            }
        }
        log!("ROM loaded into memory. Size: {} bytes", start);
        start
    }

    // Globaler Handler für den CPU der je einen Cycle ausführt
    pub fn tick(&mut self) -> Output {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            self.st -= 1;
        }

        // Opcode entpacken
        let opcode = self.fetch();
        // Opcode ausführen
        self.execute(opcode)
    }

    pub fn test_set_registers(&mut self, idx: usize, data: u8) {
        self.registers[idx] = data;
    }

    pub fn test_get_registers(&mut self, idx: usize) -> u8 {
        self.registers[idx]
    }

    pub fn test_set_memory(&mut self, idx: usize, data: u8) {
        self.mem[idx] = data;
    }

    pub fn test_get_memory(&mut self, idx: usize) -> u8 {
        self.mem[idx]
    }

    pub fn test_get_stack(&mut self, idx: usize) -> u16 {
        self.stack[idx]
    }
}
