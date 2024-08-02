193use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = "windows")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD = /*****/
-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE; const /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern "system" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: HANDLE, dwMode: DWORD ) -> BOOL; } /***/
let hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/
ENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = "windows"))] fn init_vt() {} fn main() { init_vt(); let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/
"use std::ffi::{c_void, c_int, c_ulong}; #[cfg(target_os = \"windows\")] fn init_vt() { type HANDLE = *mut c_void; type DWORD = c_ulong; type BOOL = c_int; const STD_OUTPUT_HANDLE: DWORD @@@@",
"= /*****/\n-11i32 as u32; const ENABLE_PROCESSED_OUTPUT: DWORD = 0x0001; const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004; const INVALID_HANDLE_VALUE: HANDLE = -1isize as @@@@@@@@@@@",
"HANDLE; const /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT: DWORD = 0x0002; extern \"system\" { pub fn GetStdHandle( nStdHandle: DWORD, ) -> HANDLE; pub fn SetConsoleMode( hConsoleHandle: @@@@@@@@",
"HANDLE, dwMode: DWORD ) -> BOOL; } /***/\nlet hout = unsafe{GetStdHandle(STD_OUTPUT_HANDLE)}; assert!(hout != INVALID_HANDLE_VALUE); unsafe{SetConsoleMode(hout, @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT | /**/ /****/\nENABLE_WRAP_AT_EOL_OUTPUT)}; } #[cfg(not(target_os = \"windows\"))] fn init_vt() {} fn main() { init_vt(); @@@@@@@",
"let src : &str = concat!( /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /*****/\n\"`\"\n); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 @@@@@@@@@@",
"/*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/\n64 { quine.push_str(\"\\\",\\n\\\"\"); previous_at = false; } match ch as @@@@",
"u32 { 92 /*backslash*/ => quine.push_str(\"\\\\\\\\\"), 10 /*newline*/ => quine.push_str(\"\\\\n\"), 34 /*quote*/ => /**/ /**/ /**/ /**/\nquine.push_str(\"\\\\\\\"\"), 64 => @@@@@@@@@@@@@@@@",
"{previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/\nmut x= 0; const @@@@@@@@@",
"PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ @@@@@@@@@@@@@@@@@@",
"/***/\n(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ @@@@@@@",
"/**/ /**/ /*****/\n(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), @@@@@@@@@@@@@@@@@@@",
"(0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/\n(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), @@@@@@@@@@@@@@",
"(0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/\n(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in @@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"quine.chars().enumerate() { if ch == '\\n' { fire_width = i; break; } } print!(\"{}\", /**/ /***/\n&fire_width); loop { print!(\"{}\", 27 as char); print!(\"[38;2;{};{};{}m\", @@@@@@@@@@@@@@",
"PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!(\"{}\", quine); print!(\"{}[0m\", 27 as char); print!(\"{}[H\", 27 as /****/\nchar); @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@",
"std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ @@@@@@@@@@",
"/****/\n@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
); let mut quine = std::string::String::new(); for ch in src.chars() { if ch == 96 /*backtick*/ as char{ let mut previous_at = false; for ch in src.chars() { if previous_at && ch as u32 != /**/
64 { quine.push_str("\",\n\""); previous_at = false; } match ch as u32 { 92 /*backslash*/ => quine.push_str("\\\\"), 10 /*newline*/ => quine.push_str("\\n"), 34 /*quote*/ => /**/ /**/ /**/ /**/
quine.push_str("\\\""), 64 => {previous_at = true; quine.push(64 as char);}, _ => quine.push(ch), } } } else if ch == 64 /*at sign*/ as char { continue; } else { quine.push(ch); } } let /*****/
mut x= 0; const PALETTE: [(i32, i32, i32); 37]= [ (0x07,0x07,0x07,), (0x1F,0x07,0x07,), (0x2F,0x0F,0x07,), (0x47,0x0F,0x07,), (0x57,0x17,0x07,), (0x67,0x1F,0x07,), (0x77,0x1F,0x07,), /**/ /***/
(0x8F,0x27,0x07,), (0x9F,0x2F,0x07,), (0xAF,0x3F,0x07,), (0xBF,0x47,0x07,), (0xC7,0x47,0x07,), (0xDF,0x4F,0x07,), (0xDF,0x57,0x07,), (0xDF,0x57,0x07,), (0xD7,0x5F,0x07,), /**/ /**/ /**/ /*****/
(0xD7,0x5F,0x07,), (0xD7,0x67,0x0F,), (0xCF,0x6F,0x0F,), (0xCF,0x77,0x0F,), (0xCF,0x7F,0x0F,), (0xCF,0x87,0x17,), (0xC7,0x87,0x17,), (0xC7,0x8F,0x17,), (0xC7,0x97,0x1F,), /**/ /**/ /**/ /*****/
(0xBF,0x9F,0x1F,), (0xBF,0x9F,0x1F,), (0xBF,0xA7,0x27,), (0xBF,0xA7,0x27,), (0xBF,0xAF,0x2F,), (0xB7,0xAF,0x2F,), (0xB7,0xB7,0x2F,), (0xB7,0xB7,0x37,), (0xCF,0xCF,0x6F,), /**/ /**/ /**/ /*****/
(0xDF,0xDF,0x9F,), (0xEF,0xEF,0xC7,), (0xFF,0xFF,0xFF,), ]; let mut fire_width = 0; for (i, ch) in quine.chars().enumerate() { if ch == '\n' { fire_width = i; break; } } print!("{}", /**/ /***/
&fire_width); loop { print!("{}", 27 as char); print!("[38;2;{};{};{}m", PALETTE[x].0, PALETTE[x].1, PALETTE[x].2); print!("{}", quine); print!("{}[0m", 27 as char); print!("{}[H", 27 as /****/
char); std::thread::sleep(std::time::Duration::from_millis(20)); x += 1; } } /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /**/ /****/
