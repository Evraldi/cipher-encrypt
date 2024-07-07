use eframe::{egui, epi};

// Define the Vigenère cipher encryption function
fn vigenere_encrypt(message: &str, keyword: &str) -> String {
    if keyword.is_empty() || !keyword.chars().all(|c| c.is_alphabetic()) {
        return String::new();
    }
    let mut result = String::new();
    let keyword = keyword.to_uppercase();
    let keyword_length = keyword.len();
    let mut keyword_index = 0;

    for c in message.chars() {
        if c.is_alphabetic() {
            let c = c.to_ascii_uppercase();
            let k = keyword.chars().cycle().nth(keyword_index % keyword_length).unwrap();

            let encrypted_char = (((c as u8 - b'A') + (k as u8 - b'A')) % 26) + b'A';
            result.push(encrypted_char as char);

            keyword_index += 1;
        } else {
            result.push(c);
        }
    }
    result
}

// Define the Caesar cipher encryption function
fn caesar_encrypt(message: &str, shift: usize) -> String {
    if shift == 0 {
        return String::new();
    }
    let mut result = String::new();

    for c in message.chars() {
        if c.is_alphabetic() {
            let base = if c.is_uppercase() { b'A' } else { b'a' };
            let encrypted_char = (((c as u8 - base) + shift as u8) % 26) + base;
            result.push(encrypted_char as char);
        } else {
            result.push(c);
        }
    }
    result
}

// Define the Atbash cipher encryption function
fn atbash_encrypt(message: &str) -> String {
    let mut result = String::new();

    for c in message.chars() {
        if c.is_alphabetic() {
            let base = if c.is_uppercase() { b'A' } else { b'a' };
            let offset = (b'Z' - c.to_ascii_uppercase() as u8) % 26;
            let encrypted_char = base + offset;
            result.push(encrypted_char as char);
        } else {
            result.push(c);
        }
    }
    result
}

// Define the ROT13 cipher encryption function
fn rot13_encrypt(message: &str) -> String {
    caesar_encrypt(message, 13)
}

// Define the Playfair cipher encryption function
fn playfair_encrypt(message: &str, keyword: &str) -> String {
    if keyword.is_empty() || !keyword.chars().all(|c| c.is_alphabetic()) {
        return String::new();
    }
    let keyword = keyword.to_uppercase();
    let matrix = create_playfair_matrix(&keyword);
    let message = prepare_playfair_message(message);

    let mut result = String::new();
    let mut i = 0;

    while i < message.len() {
        let a = message[i];
        let b = if i + 1 < message.len() { message[i + 1] } else { 'X' }; // Add filler 'X' if needed
        i += 2;

        let (row_a, col_a) = find_position(a, &matrix);
        let (row_b, col_b) = find_position(b, &matrix);

        if row_a == row_b {
            result.push(matrix[row_a][(col_a + 1) % 5]);
            result.push(matrix[row_b][(col_b + 1) % 5]);
        } else if col_a == col_b {
            result.push(matrix[(row_a + 1) % 5][col_a]);
            result.push(matrix[(row_b + 1) % 5][col_b]);
        } else {
            result.push(matrix[row_a][col_b]);
            result.push(matrix[row_b][col_a]);
        }
    }

    result
}

fn create_playfair_matrix(keyword: &str) -> [[char; 5]; 5] {
    let mut matrix = [[' '; 5]; 5];
    let mut used = [false; 26];
    let mut index = 0;

    for c in keyword.chars() {
        if c.is_alphabetic() {
            let pos = (c as u8 - b'A') as usize;
            if !used[pos] && c != 'J' {
                used[pos] = true;
                matrix[index / 5][index % 5] = c;
                index += 1;
            }
        }
    }

    for i in 0..26 {
        let c = (b'A' + i as u8) as char;
        if c != 'J' && !used[i] {
            matrix[index / 5][index % 5] = c;
            index += 1;
        }
    }

    matrix
}

fn prepare_playfair_message(message: &str) -> Vec<char> {
    let mut message = message.to_uppercase().chars().filter(|c| c.is_alphabetic()).collect::<Vec<_>>();
    let mut i = 0;
    while i < message.len() - 1 {
        if message[i] == message[i + 1] {
            message.insert(i + 1, 'X');
        }
        i += 2;
    }
    if message.len() % 2 != 0 {
        message.push('X');
    }
    message
}

fn find_position(c: char, matrix: &[[char; 5]; 5]) -> (usize, usize) {
    for (row, row_data) in matrix.iter().enumerate() {
        for (col, &item) in row_data.iter().enumerate() {
            if item == c {
                return (row, col);
            }
        }
    }
    (0, 0)
}

// Define the application state
struct MyApp {
    message: String,
    keyword: String,
    shift: usize,
    encrypted_message: String,
    selected_cipher: Cipher,
    notification: Option<String>,
}

#[derive(Debug, PartialEq)] // Derive PartialEq for Cipher enum
enum Cipher {
    Vigenere,
    Caesar,
    Atbash,
    ROT13,
    Playfair,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {
            message: String::new(),
            keyword: String::new(),
            shift: 3,
            encrypted_message: String::new(),
            selected_cipher: Cipher::Vigenere,
            notification: None,
        }
    }
}

// Implement the epi::App trait for the application state
impl epi::App for MyApp {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Cipher Machine");

            // Cipher selection
            ui.horizontal(|ui| {
                ui.label("Select Cipher:");
                ui.radio_value(&mut self.selected_cipher, Cipher::Vigenere, "Vigenère");
                ui.radio_value(&mut self.selected_cipher, Cipher::Caesar, "Caesar");
                ui.radio_value(&mut self.selected_cipher, Cipher::Atbash, "Atbash");
                ui.radio_value(&mut self.selected_cipher, Cipher::ROT13, "ROT13");
                ui.radio_value(&mut self.selected_cipher, Cipher::Playfair, "Playfair");
            });

            // Message input
            ui.horizontal(|ui| {
                ui.label("Message:");
                ui.text_edit_multiline(&mut self.message);
            });

            // Cipher settings
            match self.selected_cipher {
                Cipher::Vigenere | Cipher::Playfair => {
                    ui.horizontal(|ui| {
                        ui.label("Keyword:");
                        ui.text_edit_singleline(&mut self.keyword);
                    });
                }
                Cipher::Caesar => {
                    ui.horizontal(|ui| {
                        ui.label("Shift:");
                        ui.add(egui::Slider::new(&mut self.shift, 1..=25));
                    });
                }
                _ => {}
            }

            // Encryption button
            if ui.button("Encrypt").clicked() {
                self.notification = None; // Clear previous notifications

                // Validation and encryption logic
                self.encrypted_message = match self.selected_cipher {
                    Cipher::Vigenere => {
                        if self.keyword.is_empty() {
                            self.notification = Some("Vigenère cipher requires a keyword. Please enter a keyword.".to_string());
                            String::new()
                        } else if !self.keyword.chars().all(|c| c.is_alphabetic()) {
                            self.notification = Some("Keyword must contain only alphabetic characters.".to_string());
                            String::new()
                        } else {
                            vigenere_encrypt(&self.message, &self.keyword)
                        }
                    }
                    Cipher::Caesar => {
                        if self.shift == 0 {
                            self.notification = Some("Caesar cipher requires a shift value between 1 and 25. Please adjust the shift value.".to_string());
                            String::new()
                        } else {
                            caesar_encrypt(&self.message, self.shift)
                        }
                    }
                    Cipher::Atbash => atbash_encrypt(&self.message),
                    Cipher::ROT13 => rot13_encrypt(&self.message),
                    Cipher::Playfair => {
                        if self.keyword.is_empty() {
                            self.notification = Some("Playfair cipher requires a keyword. Please enter a keyword.".to_string());
                            String::new()
                        } else if !self.keyword.chars().all(|c| c.is_alphabetic()) {
                            self.notification = Some("Keyword must contain only alphabetic characters.".to_string());
                            String::new()
                        } else {
                            playfair_encrypt(&self.message, &self.keyword)
                        }
                    }
                };

                // Notify user if encryption failed
                if self.encrypted_message.is_empty() && self.notification.is_some() {
                    ui.label(format!("Error: {}", self.notification.as_ref().unwrap()));
                } else if self.encrypted_message.is_empty() {
                    self.notification = Some("Encryption failed. Please check your inputs.".to_string());
                } else {
                    self.notification = Some("Encryption successful!".to_string());
                }
            }

            // Show notifications
            if let Some(notification) = &self.notification {
                ui.horizontal(|ui| {
                    ui.monospace(notification);
                });
            }

            // Encrypted message output
            ui.horizontal(|ui| {
                ui.label("Encrypted Message:");
                ui.monospace(&self.encrypted_message);
            });
        });
    }

    fn name(&self) -> &str {
        "Cipher Machine"
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    eframe::run_native(
        Box::new(MyApp::default()), // Pass the `MyApp` instance
        eframe::NativeOptions {
            initial_window_size: Some(egui::Vec2::new(600.0, 400.0)),
            ..Default::default()
        },
    )
}
