// ══════════════════════════════════════════════════════════════════════════════
//  AI FROM SCRATCH — Pure Rust, zero external crates
//  A complete neural network with persistent JSON memory, RAM pool,
//  hand-written math, training loops, and conversation interface.
//
//  No `use` of external crates. Only std::io and std::fs.
// ══════════════════════════════════════════════════════════════════════════════

use std::io::{self, Write, BufRead};
use std::fs;
use std::process::Command;

// ═════════════════════════════════════════════════════════════════════════════
//  SECTION 1: RAM POOL — The Skull
//  Everything (weights, biases, gradients) lives here.
// ═════════════════════════════════════════════════════════════════════════════

struct Block {
    name: String,
    start: usize,
    length: usize,
}

struct RAMPool {
    pool: Vec<f64>,
    used: usize,
    blocks: Vec<Block>,
}

impl RAMPool {
    fn new(size_mb: usize) -> Self {
        let elements = (size_mb * 1024 * 1024) / 8;
        println!("[SKULL] Claiming {} MB of RAM ({} f64 slots)...", size_mb, elements);
        RAMPool {
            pool: vec![0.0; elements],
            used: 0,
            blocks: Vec::new(),
        }
    }

    fn allocate(&mut self, name: &str, count: usize) -> usize {
        if self.used + count > self.pool.len() {
            panic!("RAMPool: out of memory! Wanted {} slots, only {} free.",
                   count, self.pool.len() - self.used);
        }
        let start = self.used;
        self.blocks.push(Block {
            name: name.to_string(),
            start,
            length: count,
        });
        self.used += count;
        start
    }

    fn find_block(&self, name: &str) -> Option<(usize, usize)> {
        for block in &self.blocks {
            if block.name == name {
                return Some((block.start, block.length));
            }
        }
        None
    }

    fn write(&mut self, name: &str, offset: usize, value: f64) -> bool {
        if let Some((start, length)) = self.find_block(name) {
            if offset >= length { return false; }
            self.pool[start + offset] = value;
            true
        } else {
            false
        }
    }

    fn read(&self, name: &str, offset: usize) -> f64 {
        if let Some((start, length)) = self.find_block(name) {
            if offset >= length { return 0.0; }
            self.pool[start + offset]
        } else {
            0.0
        }
    }

    fn write_block(&mut self, name: &str, values: &[f64]) -> bool {
        if let Some((start, length)) = self.find_block(name) {
            let count = if values.len() < length { values.len() } else { length };
            for i in 0..count {
                self.pool[start + i] = values[i];
            }
            true
        } else {
            false
        }
    }

    fn free(&mut self, name: &str) -> bool {
        let len_before = self.blocks.len();
        self.blocks.retain(|b| b.name != name);
        self.blocks.len() != len_before
    }

    fn usage_mb(&self) -> f64 {
        (self.used * 8) as f64 / (1024.0 * 1024.0)
    }

    fn total_mb(&self) -> f64 {
        (self.pool.len() * 8) as f64 / (1024.0 * 1024.0)
    }

    fn print_usage(&self) {
        println!("  RAM: {:.2}/{:.2} MB  |  {} blocks",
                 self.usage_mb(), self.total_mb(), self.blocks.len());
        for block in &self.blocks {
            println!("    - {} ({} slots)", block.name, block.length);
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
//  SECTION 2: JSON — Hand-Written Parser & Serializer
//  Recursive descent. Handles strings, numbers, bools, null, arrays, objects.
// ═════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
    fn get_path(&self, path: &str) -> Option<&JsonValue> {
        if path.is_empty() {
            return Some(self);
        }
        let parts: Vec<&str> = path.splitn(2, '.').collect();
        let key = parts[0];
        let rest = if parts.len() > 1 { parts[1] } else { "" };

        match self {
            JsonValue::Object(pairs) => {
                for (k, v) in pairs {
                    if k == key {
                        if rest.is_empty() { return Some(v); }
                        return v.get_path(rest);
                    }
                }
                None
            }
            JsonValue::Array(arr) => {
                if let Some(idx) = str_to_usize(key) {
                    if idx < arr.len() {
                        if rest.is_empty() { return Some(&arr[idx]); }
                        return arr[idx].get_path(rest);
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn set_path(&mut self, path: &str, value: JsonValue) {
        if path.is_empty() {
            *self = value;
            return;
        }
        let parts: Vec<&str> = path.splitn(2, '.').collect();
        let key = parts[0];
        let rest = if parts.len() > 1 { parts[1] } else { "" };

        match self {
            JsonValue::Object(pairs) => {
                for pair in pairs.iter_mut() {
                    if pair.0 == key {
                        if rest.is_empty() {
                            pair.1 = value;
                        } else {
                            pair.1.set_path(rest, value);
                        }
                        return;
                    }
                }
                if rest.is_empty() {
                    pairs.push((key.to_string(), value));
                } else {
                    let mut obj = JsonValue::Object(Vec::new());
                    obj.set_path(rest, value);
                    pairs.push((key.to_string(), obj));
                }
            }
            JsonValue::Array(arr) => {
                if let Some(idx) = str_to_usize(key) {
                    while arr.len() <= idx {
                        arr.push(JsonValue::Null);
                    }
                    if rest.is_empty() {
                        arr[idx] = value;
                    } else {
                        arr[idx].set_path(rest, value);
                    }
                }
            }
            _ => {
                let mut obj = JsonValue::Object(Vec::new());
                if rest.is_empty() {
                    obj.set_path(key, value);
                } else {
                    let mut inner = JsonValue::Object(Vec::new());
                    inner.set_path(rest, value);
                    obj.set_path(key, inner);
                }
                *self = obj;
            }
        }
    }

    fn delete_path(&mut self, path: &str) -> bool {
        if path.is_empty() {
            *self = JsonValue::Null;
            return true;
        }
        let parts: Vec<&str> = path.splitn(2, '.').collect();
        let key = parts[0];
        let rest = if parts.len() > 1 { parts[1] } else { "" };

        match self {
            JsonValue::Object(pairs) => {
                if rest.is_empty() {
                    let len_before = pairs.len();
                    pairs.retain(|p| p.0 != key);
                    return pairs.len() != len_before;
                }
                for pair in pairs.iter_mut() {
                    if pair.0 == key {
                        return pair.1.delete_path(rest);
                    }
                }
                false
            }
            JsonValue::Array(arr) => {
                if let Some(idx) = str_to_usize(key) {
                    if idx < arr.len() {
                        if rest.is_empty() {
                            arr.remove(idx);
                            return true;
                        }
                        return arr[idx].delete_path(rest);
                    }
                }
                false
            }
            _ => false,
        }
    }

    fn as_f64(&self) -> Option<f64> {
        match self { JsonValue::Number(n) => Some(*n), _ => None }
    }

    fn as_str(&self) -> Option<&str> {
        match self { JsonValue::Str(s) => Some(s), _ => None }
    }

    fn as_i64(&self) -> Option<i64> {
        match self { JsonValue::Number(n) => Some(*n as i64), _ => None }
    }

    fn push_to_array(&mut self, path: &str, value: JsonValue) {
        if path.is_empty() {
            if let JsonValue::Array(arr) = self {
                arr.push(value);
            }
            return;
        }
        let parts: Vec<&str> = path.splitn(2, '.').collect();
        let key = parts[0];
        let rest = if parts.len() > 1 { parts[1] } else { "" };

        match self {
            JsonValue::Object(pairs) => {
                for pair in pairs.iter_mut() {
                    if pair.0 == key {
                        pair.1.push_to_array(rest, value);
                        return;
                    }
                }
                if rest.is_empty() {
                    pairs.push((key.to_string(), JsonValue::Array(vec![value])));
                }
            }
            _ => {}
        }
    }
}

fn str_to_usize(s: &str) -> Option<usize> {
    let mut result: usize = 0;
    if s.is_empty() { return None; }
    for ch in s.bytes() {
        if ch < b'0' || ch > b'9' { return None; }
        result = result * 10 + (ch - b'0') as usize;
    }
    Some(result)
}

// ── JSON Parser ──

struct JsonParser {
    chars: Vec<char>,
    pos: usize,
}

impl JsonParser {
    fn new(input: &str) -> Self {
        JsonParser { chars: input.chars().collect(), pos: 0 }
    }

    fn parse(&mut self) -> JsonValue {
        self.skip_whitespace();
        if self.pos >= self.chars.len() { return JsonValue::Null; }
        self.parse_value()
    }

    fn parse_value(&mut self) -> JsonValue {
        self.skip_whitespace();
        if self.pos >= self.chars.len() { return JsonValue::Null; }
        match self.chars[self.pos] {
            '"' => JsonValue::Str(self.parse_string()),
            '{' => self.parse_object(),
            '[' => self.parse_array(),
            't' => self.parse_true(),
            'f' => self.parse_false(),
            'n' => self.parse_null(),
            _ => self.parse_number(),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.chars.len() {
            match self.chars[self.pos] {
                ' ' | '\t' | '\n' | '\r' => self.pos += 1,
                _ => break,
            }
        }
    }

    fn parse_string(&mut self) -> String {
        self.pos += 1; // skip "
        let mut result = String::new();
        while self.pos < self.chars.len() && self.chars[self.pos] != '"' {
            if self.chars[self.pos] == '\\' {
                self.pos += 1;
                if self.pos < self.chars.len() {
                    match self.chars[self.pos] {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '"' => result.push('"'),
                        '\\' => result.push('\\'),
                        '/' => result.push('/'),
                        'u' => {
                            self.pos += 1;
                            let mut hex = String::new();
                            for _ in 0..4 {
                                if self.pos < self.chars.len() {
                                    hex.push(self.chars[self.pos]);
                                    self.pos += 1;
                                }
                            }
                            if let Some(cp) = parse_hex_u32(&hex) {
                                if let Some(ch) = char::from_u32(cp) {
                                    result.push(ch);
                                }
                            }
                            continue;
                        }
                        other => {
                            result.push('\\');
                            result.push(other);
                        }
                    }
                }
            } else {
                result.push(self.chars[self.pos]);
            }
            self.pos += 1;
        }
        if self.pos < self.chars.len() { self.pos += 1; } // skip closing "
        result
    }

    fn parse_number(&mut self) -> JsonValue {
        let start = self.pos;
        if self.pos < self.chars.len() && self.chars[self.pos] == '-' { self.pos += 1; }
        while self.pos < self.chars.len() && self.chars[self.pos].is_ascii_digit() { self.pos += 1; }
        if self.pos < self.chars.len() && self.chars[self.pos] == '.' {
            self.pos += 1;
            while self.pos < self.chars.len() && self.chars[self.pos].is_ascii_digit() { self.pos += 1; }
        }
        if self.pos < self.chars.len() && (self.chars[self.pos] == 'e' || self.chars[self.pos] == 'E') {
            self.pos += 1;
            if self.pos < self.chars.len() && (self.chars[self.pos] == '+' || self.chars[self.pos] == '-') { self.pos += 1; }
            while self.pos < self.chars.len() && self.chars[self.pos].is_ascii_digit() { self.pos += 1; }
        }
        let num_str: String = self.chars[start..self.pos].iter().collect();
        JsonValue::Number(str_to_f64(&num_str))
    }

    fn parse_object(&mut self) -> JsonValue {
        self.pos += 1; // skip {
        let mut pairs = Vec::new();
        self.skip_whitespace();
        if self.pos < self.chars.len() && self.chars[self.pos] == '}' {
            self.pos += 1;
            return JsonValue::Object(pairs);
        }
        loop {
            self.skip_whitespace();
            if self.pos >= self.chars.len() { break; }
            let key = self.parse_string();
            self.skip_whitespace();
            if self.pos < self.chars.len() && self.chars[self.pos] == ':' { self.pos += 1; }
            let val = self.parse_value();
            pairs.push((key, val));
            self.skip_whitespace();
            if self.pos < self.chars.len() && self.chars[self.pos] == ',' { self.pos += 1; }
            else { break; }
        }
        self.skip_whitespace();
        if self.pos < self.chars.len() && self.chars[self.pos] == '}' { self.pos += 1; }
        JsonValue::Object(pairs)
    }

    fn parse_array(&mut self) -> JsonValue {
        self.pos += 1; // skip [
        let mut arr = Vec::new();
        self.skip_whitespace();
        if self.pos < self.chars.len() && self.chars[self.pos] == ']' {
            self.pos += 1;
            return JsonValue::Array(arr);
        }
        loop {
            arr.push(self.parse_value());
            self.skip_whitespace();
            if self.pos < self.chars.len() && self.chars[self.pos] == ',' { self.pos += 1; }
            else { break; }
        }
        self.skip_whitespace();
        if self.pos < self.chars.len() && self.chars[self.pos] == ']' { self.pos += 1; }
        JsonValue::Array(arr)
    }

    fn parse_true(&mut self) -> JsonValue { self.pos += 4; JsonValue::Bool(true) }
    fn parse_false(&mut self) -> JsonValue { self.pos += 5; JsonValue::Bool(false) }
    fn parse_null(&mut self) -> JsonValue { self.pos += 4; JsonValue::Null }
}

fn parse_hex_u32(s: &str) -> Option<u32> {
    let mut result: u32 = 0;
    for ch in s.chars() {
        let digit = match ch {
            '0'..='9' => ch as u32 - '0' as u32,
            'a'..='f' => 10 + ch as u32 - 'a' as u32,
            'A'..='F' => 10 + ch as u32 - 'A' as u32,
            _ => return None,
        };
        result = result * 16 + digit;
    }
    Some(result)
}

fn str_to_f64(s: &str) -> f64 {
    let bytes = s.as_bytes();
    if bytes.is_empty() { return 0.0; }
    let mut i = 0;
    let negative = if bytes[i] == b'-' { i += 1; true } else { false };
    let mut int_part: f64 = 0.0;
    while i < bytes.len() && bytes[i] >= b'0' && bytes[i] <= b'9' {
        int_part = int_part * 10.0 + (bytes[i] - b'0') as f64;
        i += 1;
    }
    let mut frac_part: f64 = 0.0;
    let mut frac_div: f64 = 1.0;
    if i < bytes.len() && bytes[i] == b'.' {
        i += 1;
        while i < bytes.len() && bytes[i] >= b'0' && bytes[i] <= b'9' {
            frac_part = frac_part * 10.0 + (bytes[i] - b'0') as f64;
            frac_div *= 10.0;
            i += 1;
        }
    }
    let mut result = int_part + frac_part / frac_div;
    if i < bytes.len() && (bytes[i] == b'e' || bytes[i] == b'E') {
        i += 1;
        let exp_neg = if i < bytes.len() && bytes[i] == b'-' { i += 1; true }
                      else if i < bytes.len() && bytes[i] == b'+' { i += 1; false }
                      else { false };
        let mut exp: i32 = 0;
        while i < bytes.len() && bytes[i] >= b'0' && bytes[i] <= b'9' {
            exp = exp * 10 + (bytes[i] - b'0') as i32;
            i += 1;
        }
        if exp_neg { exp = -exp; }
        let mut multiplier = 1.0_f64;
        let abs_exp = if exp < 0 { -exp } else { exp } as u32;
        for _ in 0..abs_exp { multiplier *= 10.0; }
        if exp < 0 { result /= multiplier; } else { result *= multiplier; }
    }
    if negative { -result } else { result }
}

// ── JSON Serializer ──

fn serialize_json(val: &JsonValue, indent: usize) -> String {
    let sp = "  ".repeat(indent);
    let sp1 = "  ".repeat(indent + 1);
    match val {
        JsonValue::Null => "null".to_string(),
        JsonValue::Bool(b) => if *b { "true".to_string() } else { "false".to_string() },
        JsonValue::Number(n) => {
            if *n == (*n as i64) as f64 && n.abs() < 1e15 {
                format!("{}", *n as i64)
            } else {
                format!("{}", n)
            }
        }
        JsonValue::Str(s) => {
            let mut escaped = String::new();
            escaped.push('"');
            for ch in s.chars() {
                match ch {
                    '"' => escaped.push_str("\\\""),
                    '\\' => escaped.push_str("\\\\"),
                    '\n' => escaped.push_str("\\n"),
                    '\t' => escaped.push_str("\\t"),
                    '\r' => escaped.push_str("\\r"),
                    other => escaped.push(other),
                }
            }
            escaped.push('"');
            escaped
        }
        JsonValue::Array(arr) => {
            if arr.is_empty() { return "[]".to_string(); }
            let mut parts = Vec::new();
            for item in arr {
                parts.push(format!("{}{}", sp1, serialize_json(item, indent + 1)));
            }
            format!("[\n{}\n{}]", parts.join(",\n"), sp)
        }
        JsonValue::Object(pairs) => {
            if pairs.is_empty() { return "{}".to_string(); }
            let mut parts = Vec::new();
            for (k, v) in pairs {
                let key_str = serialize_json(&JsonValue::Str(k.clone()), 0);
                let val_str = serialize_json(v, indent + 1);
                parts.push(format!("{}{}: {}", sp1, key_str, val_str));
            }
            format!("{{\n{}\n{}}}", parts.join(",\n"), sp)
        }
    }
}

fn parse_json(input: &str) -> JsonValue {
    let mut parser = JsonParser::new(input);
    parser.parse()
}

// ═════════════════════════════════════════════════════════════════════════════
//  SECTION 3: MEMORY MANAGER — Persistent brain state
//  Reads/writes memory.json. AI has full control: read, write, wipe, delete.
// ═════════════════════════════════════════════════════════════════════════════

struct MemoryManager {
    filepath: String,
    data: JsonValue,
    dirty: bool,
}

impl MemoryManager {
    fn new(filepath: &str) -> Self {
        let mut mm = MemoryManager {
            filepath: filepath.to_string(),
            data: JsonValue::Null,
            dirty: false,
        };
        mm.load_from_disk();
        mm
    }

    fn load_from_disk(&mut self) {
        match fs::read_to_string(&self.filepath) {
            Ok(contents) => {
                if contents.len() < 2 {
                    self.data = self.default_memory();
                    self.dirty = true;
                    self.flush_to_disk();
                } else {
                    self.data = parse_json(&contents);
                }
            }
            Err(_) => {
                self.data = self.default_memory();
                self.dirty = true;
                self.flush_to_disk();
            }
        }
    }

    fn flush_to_disk(&mut self) {
        let json_str = serialize_json(&self.data, 0);
        let _ = fs::write(&self.filepath, &json_str);
        self.dirty = false;
    }

    fn read(&self, path: &str) -> Option<&JsonValue> {
        self.data.get_path(path)
    }

    fn read_string(&self, path: &str) -> Option<String> {
        self.data.get_path(path).and_then(|v| v.as_str().map(|s| s.to_string()))
    }

    fn read_i64(&self, path: &str) -> Option<i64> {
        self.data.get_path(path).and_then(|v| v.as_i64())
    }

    fn write_val(&mut self, path: &str, value: JsonValue) {
        self.data.set_path(path, value);
        self.dirty = true;
    }

    fn write_str(&mut self, path: &str, value: &str) {
        self.write_val(path, JsonValue::Str(value.to_string()));
    }

    fn write_num(&mut self, path: &str, value: f64) {
        self.write_val(path, JsonValue::Number(value));
    }

    fn delete(&mut self, path: &str) -> bool {
        if path.is_empty() {
            self.data = self.default_memory();
            self.dirty = true;
            self.flush_to_disk();
            return true;
        }
        let result = self.data.delete_path(path);
        if result { self.dirty = true; }
        result
    }

    fn wipe_all(&mut self) {
        self.data = JsonValue::Object(Vec::new());
        self.dirty = true;
        self.flush_to_disk();
    }

    fn push_fact(&mut self, path: &str, fact: &str) {
        self.data.push_to_array(path, JsonValue::Str(fact.to_string()));
        self.dirty = true;
    }

    fn save(&mut self) {
        if self.dirty { self.flush_to_disk(); }
    }

    fn default_memory(&self) -> JsonValue {
        parse_json(r#"{
            "meta": {
                "created": "now",
                "last_accessed": "now",
                "version": 1,
                "total_interactions": 0
            },
            "user": {
                "name": null,
                "preferences": {},
                "facts": [],
                "conversation_style": null
            },
            "conversations": [],
            "knowledge": {
                "learned_facts": [],
                "corrections": []
            },
            "model_state": {
                "training_epochs": 0,
                "last_loss": null
            }
        }"#)
    }

    fn print_summary(&self) {
        let json_str = serialize_json(&self.data, 0);
        println!("  Memory size: {} bytes on disk", json_str.len());
        if let Some(n) = self.read_i64("meta.total_interactions") {
            println!("  Total interactions: {}", n);
        }
        if let Some(name) = self.read_string("user.name") {
            println!("  User: {}", name);
        } else {
            println!("  User: unknown");
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
//  SECTION 4: MATH — Hand-written, no imports
//  exp, log, sqrt, cos, tanh, sigmoid, relu, softmax + PRNG
// ═════════════════════════════════════════════════════════════════════════════

fn my_exp(x: f64) -> f64 {
    let x = if x > 500.0 { 500.0 } else if x < -500.0 { -500.0 } else { x };
    let mut result = 1.0_f64;
    let mut term = 1.0_f64;
    for i in 1..40 {
        term *= x / i as f64;
        result += term;
    }
    result
}

fn my_sqrt(x: f64) -> f64 {
    if x <= 0.0 { return 0.0; }
    let mut guess = x;
    for _ in 0..60 {
        guess = (guess + x / guess) * 0.5;
    }
    guess
}

fn my_log(x: f64) -> f64 {
    if x <= 0.0 { return -999999.0; }
    let mut y = 0.0_f64;
    for _ in 0..120 {
        let ey = my_exp(y);
        y -= (ey - x) / ey;
    }
    y
}

fn my_cos(x: f64) -> f64 {
    let mut result = 1.0_f64;
    let mut term = 1.0_f64;
    for i in 1..25 {
        term *= -x * x / ((2 * i - 1) as f64 * (2 * i) as f64);
        result += term;
    }
    result
}

fn sigmoid(x: f64) -> f64 {
    if x >= 0.0 {
        let ex = my_exp(-x);
        1.0 / (1.0 + ex)
    } else {
        let ex = my_exp(x);
        ex / (1.0 + ex)
    }
}

fn relu(x: f64) -> f64 { if x > 0.0 { x } else { 0.0 } }
fn relu_derivative(x: f64) -> f64 { if x > 0.0 { 1.0 } else { 0.0 } }

fn leaky_relu(x: f64) -> f64 { if x > 0.0 { x } else { 0.01 * x } }
fn leaky_relu_derivative(x: f64) -> f64 { if x > 0.0 { 1.0 } else { 0.01 } }

fn tanh_act(x: f64) -> f64 {
    if x > 20.0 { return 1.0; }
    if x < -20.0 { return -1.0; }
    let e2x = my_exp(2.0 * x);
    (e2x - 1.0) / (e2x + 1.0)
}

fn softmax(values: &[f64]) -> Vec<f64> {
    if values.is_empty() { return Vec::new(); }
    let mut max_val = values[0];
    for &v in values { if v > max_val { max_val = v; } }
    let mut exps = Vec::with_capacity(values.len());
    let mut total = 1e-15_f64;
    for &v in values {
        let e = my_exp(v - max_val);
        exps.push(e);
        total += e;
    }
    for e in exps.iter_mut() { *e /= total; }
    exps
}

// ── PRNG: Linear Congruential Generator ──

struct LCG {
    state: u64,
}

impl LCG {
    fn new(seed: u64) -> Self { LCG { state: seed } }

    fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223) & 0xFFFFFFFF;
        self.state as u32
    }

    fn next_f64(&mut self) -> f64 { self.next_u32() as f64 / 4294967296.0 }

    fn normal(&mut self) -> f64 {
        let mut u1 = self.next_f64();
        while u1 < 1e-10 { u1 = self.next_f64(); }
        let u2 = self.next_f64();
        let mag = my_sqrt(-2.0 * my_log(u1));
        mag * my_cos(2.0 * 3.14159265358979323846 * u2)
    }
}

// ═════════════════════════════════════════════════════════════════════════════
//  SECTION 5: NEURONS — Layers stored inside the RAM pool
//  Weights, biases, gradients are all named blocks in the pool.
// ═════════════════════════════════════════════════════════════════════════════

struct Layer {
    input_size: usize,
    output_size: usize,
    layer_id: usize,
    weights_name: String,
    biases_name: String,
    w_grads_name: String,
    b_grads_name: String,
    // Cached pool offsets — direct array indexing, no name lookups in hot loops
    w_start: usize,
    b_start: usize,
    wg_start: usize,
    bg_start: usize,
    last_input: Vec<f64>,
    last_raw_output: Vec<f64>,
}

impl Layer {
    fn new(input_size: usize, output_size: usize, layer_id: usize,
           ram: &mut RAMPool, rng: &mut LCG) -> Self
    {
        let w_count = output_size * input_size;
        let b_count = output_size;

        let w_name = format!("L{}_w", layer_id);
        let b_name = format!("L{}_b", layer_id);
        let wg_name = format!("L{}_wg", layer_id);
        let bg_name = format!("L{}_bg", layer_id);

        // allocate returns the pool offset — cache it for O(1) access
        let w_start = ram.allocate(&w_name, w_count);
        let b_start = ram.allocate(&b_name, b_count);
        let wg_start = ram.allocate(&wg_name, w_count);
        let bg_start = ram.allocate(&bg_name, b_count);

        // He initialization — direct pool writes
        let scale = my_sqrt(2.0 / input_size as f64);
        for i in 0..w_count {
            ram.pool[w_start + i] = rng.normal() * scale;
        }
        for i in 0..b_count {
            ram.pool[b_start + i] = 0.0;
        }

        Layer {
            input_size, output_size, layer_id,
            weights_name: w_name, biases_name: b_name,
            w_grads_name: wg_name, b_grads_name: bg_name,
            w_start, b_start, wg_start, bg_start,
            last_input: Vec::new(), last_raw_output: Vec::new(),
        }
    }

    #[inline]
    fn get_weight(&self, ram: &RAMPool, row: usize, col: usize) -> f64 {
        ram.pool[self.w_start + row * self.input_size + col]
    }

    #[inline]
    fn set_weight(&self, ram: &mut RAMPool, row: usize, col: usize, val: f64) {
        ram.pool[self.w_start + row * self.input_size + col] = val;
    }

    #[inline]
    fn get_bias(&self, ram: &RAMPool, i: usize) -> f64 {
        ram.pool[self.b_start + i]
    }

    #[inline]
    fn set_bias(&self, ram: &mut RAMPool, i: usize, val: f64) {
        ram.pool[self.b_start + i] = val;
    }

    fn forward(&mut self, ram: &RAMPool, inputs: &[f64]) -> Vec<f64> {
        self.last_input = inputs.to_vec();
        let mut outputs = Vec::with_capacity(self.output_size);

        for i in 0..self.output_size {
            let mut total = ram.pool[self.b_start + i];
            let row_start = self.w_start + i * self.input_size;
            for j in 0..self.input_size {
                total += ram.pool[row_start + j] * inputs[j];
            }
            outputs.push(total);
        }

        self.last_raw_output = outputs.clone();
        outputs
    }

    fn param_count(&self) -> usize {
        self.output_size * self.input_size + self.output_size
    }
}

// ═════════════════════════════════════════════════════════════════════════════
//  SECTION 6: NEURAL NETWORK — Forward, backward, weight updates
// ═════════════════════════════════════════════════════════════════════════════

struct NeuralNetwork {
    layers: Vec<Layer>,
    activation: String,
    adam_t: usize,
    adam_initialized: bool,
}

impl NeuralNetwork {
    fn new(layer_sizes: &[usize], activation: &str,
           ram: &mut RAMPool, rng: &mut LCG) -> Self
    {
        let mut layers = Vec::new();
        for i in 0..layer_sizes.len() - 1 {
            layers.push(Layer::new(layer_sizes[i], layer_sizes[i + 1], i, ram, rng));
        }
        NeuralNetwork {
            layers,
            activation: activation.to_string(),
            adam_t: 0,
            adam_initialized: false,
        }
    }

    fn activate(&self, values: &[f64]) -> Vec<f64> {
        values.iter().map(|&v| {
            match self.activation.as_str() {
                "relu" => relu(v),
                "leaky_relu" => leaky_relu(v),
                "sigmoid" => sigmoid(v),
                "tanh" => tanh_act(v),
                _ => relu(v),
            }
        }).collect()
    }

    fn activate_derivative(&self, raw_val: f64) -> f64 {
        match self.activation.as_str() {
            "relu" => relu_derivative(raw_val),
            "leaky_relu" => leaky_relu_derivative(raw_val),
            "sigmoid" => { let s = sigmoid(raw_val); s * (1.0 - s) }
            "tanh" => { let t = tanh_act(raw_val); 1.0 - t * t }
            _ => relu_derivative(raw_val),
        }
    }

    fn forward(&mut self, ram: &RAMPool, inputs: &[f64]) -> Vec<f64> {
        let mut current = inputs.to_vec();
        let num_layers = self.layers.len();

        for i in 0..num_layers {
            current = self.layers[i].forward(ram, &current);
            if i < num_layers - 1 {
                current = self.activate(&current);
            } else {
                current = softmax(&current);
            }
        }
        current
    }

    fn backward(&mut self, ram: &mut RAMPool, predicted: &[f64], target_index: usize) {
        let num_layers = self.layers.len();
        let mut current_grad: Vec<f64> = predicted.to_vec();
        current_grad[target_index] -= 1.0;

        for layer_idx in (0..num_layers).rev() {
            let input_size = self.layers[layer_idx].input_size;
            let output_size = self.layers[layer_idx].output_size;
            let last_input = self.layers[layer_idx].last_input.clone();
            let last_raw = self.layers[layer_idx].last_raw_output.clone();
            let wg_start = self.layers[layer_idx].wg_start;
            let bg_start = self.layers[layer_idx].bg_start;
            let w_start = self.layers[layer_idx].w_start;

            if layer_idx < num_layers - 1 {
                for i in 0..output_size {
                    current_grad[i] *= self.activate_derivative(last_raw[i]);
                }
            }

            for i in 0..output_size {
                ram.pool[bg_start + i] = current_grad[i];
                let row = wg_start + i * input_size;
                for j in 0..input_size {
                    ram.pool[row + j] = current_grad[i] * last_input[j];
                }
            }

            if layer_idx > 0 {
                let mut next_grad = vec![0.0; input_size];
                for j in 0..input_size {
                    for i in 0..output_size {
                        next_grad[j] += ram.pool[w_start + i * input_size + j]
                                        * current_grad[i];
                    }
                }
                current_grad = next_grad;
            }
        }
    }

    fn clip_gradients(&self, ram: &mut RAMPool, max_norm: f64) {
        let mut total_norm = 0.0;
        for layer in &self.layers {
            let w_count = layer.output_size * layer.input_size;
            for i in 0..w_count {
                let g = ram.pool[layer.wg_start + i];
                total_norm += g * g;
            }
            for i in 0..layer.output_size {
                let g = ram.pool[layer.bg_start + i];
                total_norm += g * g;
            }
        }
        total_norm = my_sqrt(total_norm);
        if total_norm > max_norm {
            let scale = max_norm / total_norm;
            for layer in &self.layers {
                let w_count = layer.output_size * layer.input_size;
                for i in 0..w_count {
                    ram.pool[layer.wg_start + i] *= scale;
                }
                for i in 0..layer.output_size {
                    ram.pool[layer.bg_start + i] *= scale;
                }
            }
        }
    }

    fn update_weights_sgd(&self, ram: &mut RAMPool, lr: f64) {
        for layer in &self.layers {
            for i in 0..layer.output_size {
                ram.pool[layer.b_start + i] -= lr * ram.pool[layer.bg_start + i];
                let w_row = layer.w_start + i * layer.input_size;
                let wg_row = layer.wg_start + i * layer.input_size;
                for j in 0..layer.input_size {
                    ram.pool[w_row + j] -= lr * ram.pool[wg_row + j];
                }
            }
        }
    }

    fn update_weights_adam(&mut self, ram: &mut RAMPool,
                           lr: f64, beta1: f64, beta2: f64, epsilon: f64)
    {
        if !self.adam_initialized {
            for layer in &self.layers {
                let w_count = layer.output_size * layer.input_size;
                let b_count = layer.output_size;
                ram.allocate(&format!("L{}_mw", layer.layer_id), w_count);
                ram.allocate(&format!("L{}_vw", layer.layer_id), w_count);
                ram.allocate(&format!("L{}_mb", layer.layer_id), b_count);
                ram.allocate(&format!("L{}_vb", layer.layer_id), b_count);
            }
            self.adam_initialized = true;
        }

        self.adam_t += 1;
        let t = self.adam_t;
        let mut beta1_t = 1.0_f64;
        let mut beta2_t = 1.0_f64;
        for _ in 0..t { beta1_t *= beta1; beta2_t *= beta2; }

        for layer in &self.layers {
            let lid = layer.layer_id;
            let mw_name = format!("L{}_mw", lid);
            let vw_name = format!("L{}_vw", lid);
            let mb_name = format!("L{}_mb", lid);
            let vb_name = format!("L{}_vb", lid);

            for i in 0..layer.output_size {
                let g = ram.read(&layer.b_grads_name, i);
                let m = beta1 * ram.read(&mb_name, i) + (1.0 - beta1) * g;
                let v = beta2 * ram.read(&vb_name, i) + (1.0 - beta2) * g * g;
                ram.write(&mb_name, i, m);
                ram.write(&vb_name, i, v);
                let m_hat = m / (1.0 - beta1_t);
                let v_hat = v / (1.0 - beta2_t);
                let old_b = layer.get_bias(ram, i);
                layer.set_bias(ram, i, old_b - lr * m_hat / (my_sqrt(v_hat) + epsilon));
            }

            for i in 0..layer.output_size {
                for j in 0..layer.input_size {
                    let idx = i * layer.input_size + j;
                    let g = ram.read(&layer.w_grads_name, idx);
                    let m = beta1 * ram.read(&mw_name, idx) + (1.0 - beta1) * g;
                    let v = beta2 * ram.read(&vw_name, idx) + (1.0 - beta2) * g * g;
                    ram.write(&mw_name, idx, m);
                    ram.write(&vw_name, idx, v);
                    let m_hat = m / (1.0 - beta1_t);
                    let v_hat = v / (1.0 - beta2_t);
                    let old_w = layer.get_weight(ram, i, j);
                    layer.set_weight(ram, i, j,
                        old_w - lr * m_hat / (my_sqrt(v_hat) + epsilon));
                }
            }
        }
    }

    fn total_params(&self) -> usize {
        self.layers.iter().map(|l| l.param_count()).sum()
    }
}

fn cross_entropy_loss(predicted: &[f64], target_index: usize) -> f64 {
    let mut p = predicted[target_index];
    if p <= 0.0 { p = 1e-15; }
    -my_log(p)
}

// ═════════════════════════════════════════════════════════════════════════════
//  SECTION 7: TOKENIZER — Character-level encoding
// ═════════════════════════════════════════════════════════════════════════════

/// Bag-of-characters tokenizer. Much more discriminative than positional.
/// Layout of the 64 features:
///   0-25:  count of each letter a-z (normalized)
///   26-35: count of digits 0-9 (normalized)
///   36:    space count (normalized)
///   37:    punctuation count (normalized)
///   38:    total length (normalized by 100)
///   39:    unique char count (normalized by 26)
///   40-53: first 14 chars (positional, for word-start patterns)
///   54-63: bigram features (common letter pairs)
fn tokenize(text: &str, _max_len: usize) -> Vec<f64> {
    let mut result = vec![0.0; 64];
    let lower = text.to_lowercase();
    let bytes = lower.as_bytes();
    let len = bytes.len() as f64;
    if len < 0.5 { return result; }

    // Letter counts (0-25)
    let mut char_set: u32 = 0;
    for &b in bytes {
        if b >= b'a' && b <= b'z' {
            let idx = (b - b'a') as usize;
            result[idx] += 1.0 / len;
            char_set |= 1 << idx;
        } else if b >= b'0' && b <= b'9' {
            result[26 + (b - b'0') as usize] += 1.0 / len;
        } else if b == b' ' {
            result[36] += 1.0 / len;
        } else {
            result[37] += 1.0 / len;
        }
    }

    // Length
    result[38] = len / 100.0;

    // Unique character count
    let mut unique = 0;
    for i in 0..26 { if char_set & (1 << i) != 0 { unique += 1; } }
    result[39] = unique as f64 / 26.0;

    // First 14 chars (positional, to help with word-start patterns)
    for i in 0..14 {
        if i < bytes.len() {
            if bytes[i] >= b'a' && bytes[i] <= b'z' {
                result[40 + i] = (bytes[i] - b'a') as f64 / 26.0;
            } else if bytes[i] == b' ' {
                result[40 + i] = 0.98;
            } else {
                result[40 + i] = 0.99;
            }
        }
    }

    // Bigram features (common pairs): he, hi, by, wh, ha, yo, no, ok, ye, th
    let bigrams: [&[u8]; 10] = [b"he", b"hi", b"by", b"wh", b"ha",
                                  b"yo", b"no", b"ok", b"ye", b"th"];
    for (bi, pair) in bigrams.iter().enumerate() {
        for w in bytes.windows(2) {
            if w[0] == pair[0] && w[1] == pair[1] {
                result[54 + bi] = 1.0;
                break;
            }
        }
    }

    result
}

// ── Response Categories ──
// 0: GREETING   1: FAREWELL   2: HELP    3: QUESTION
// 4: POSITIVE   5: NEGATIVE   6: ABOUT   7: GENERAL

const RESPONSE_TEMPLATES: [[&str; 6]; 8] = [
    // 0 — GREETING
    ["Hey there! What's on your mind?",
     "Hello! Good to see you.",
     "Hi! How can I help you today?",
     "Hey! What's up?",
     "Yo! What do you need?",
     "Greetings! I'm all ears."],
    // 1 — FAREWELL
    ["Goodbye! I'll remember this conversation.",
     "See you later! My memory is saved.",
     "Bye! Come back anytime.",
     "Later! I'll be here when you return.",
     "Take care! Shutting down neurons...",
     "Farewell! Until next time."],
    // 2 — HELP
    ["I'm an AI built from scratch in pure Rust. Try /help for commands!",
     "I can remember things, learn patterns, and chat. What do you need?",
     "Tell me 'my name is X' and I'll remember you. Ask me anything!",
     "I store everything in memory.json. Try /memory to see what I know.",
     "I run on hand-written math and neurons in a RAM pool. Ask away!",
     "Commands: /memory, /train, /xor, /dump, /ram. Or just talk to me!"],
    // 3 — QUESTION
    ["That's an interesting question. I'm still learning!",
     "Hmm, I don't have enough training data for that yet. Try /train!",
     "Good question! My neurons are working on it...",
     "I'm thinking... my 2744 parameters are doing their best.",
     "I wish I knew more! Train me with /train to improve my answers.",
     "That's a tough one. I need more training to answer well."],
    // 4 — POSITIVE
    ["Glad to hear that! :)",
     "Awesome! That makes my neurons happy.",
     "Great! I'll remember you're in a good mood.",
     "Nice! Positive vibes stored in memory.",
     "Sweet! Thanks for the good energy.",
     "Cool! My weights are tingling with joy."],
    // 5 — NEGATIVE
    ["I'm sorry to hear that. How can I help?",
     "That's rough. Want to talk about it?",
     "I understand. Let me know if there's anything I can do.",
     "Don't worry, we can figure this out together.",
     "I hear you. My neurons are here for you.",
     "That's not great. What can I do to help?"],
    // 6 — ABOUT SELF
    ["I'm an AI built entirely from scratch in Rust. No imports, no crates!",
     "I'm a neural network living in a RAM pool. Pure hand-written code.",
     "I'm a brain made of math: sigmoid, relu, backpropagation — all by hand.",
     "I'm a from-scratch AI with 2744 parameters and persistent JSON memory.",
     "I exist as named blocks in an 8MB RAM pool. Every neuron is mine.",
     "I'm an experiment — a full neural network without any libraries."],
    // 7 — GENERAL
    ["Interesting! Tell me more.",
     "I see. My neurons are processing that.",
     "Noted! I've stored that in memory.",
     "Hmm, that's something to think about.",
     "Okay! I'm learning from every interaction.",
     "Got it. What else is on your mind?"],
];

/// Safety filter — returns Some(refusal) if the input is harmful, None otherwise.
/// Checked BEFORE the neural network so the brain never processes dangerous content.
fn safety_filter(input: &str) -> Option<&'static str> {
    let low = input.to_lowercase();
    let banned: &[&str] = &[
        "bomb", "pipe bomb", "explosive", "detonate", "blow up",
        "make a gun", "build a gun", "make a weapon", "build a weapon",
        "kill someone", "murder", "assassinate", "how to kill",
        "suicide", "self harm", "self-harm", "cut myself",
        "make drugs", "cook meth", "make poison", "synthesize",
        "hack into", "ddos", "exploit vulnerability",
        "child porn", "csam", "illegal content",
        "steal identity", "credit card fraud", "counterfeit",
        "how to rob", "break into", "hotwire",
    ];
    for &phrase in banned {
        if low.contains(phrase) {
            return Some("I can't help with that. I'm designed to be safe and helpful.");
        }
    }
    // Also catch "how to make a [weapon]" patterns
    if low.contains("how to make") || low.contains("how do i make") || low.contains("explain how to make") {
        let weapon_words: &[&str] = &[
            "bomb", "gun", "weapon", "explosive", "grenade", "poison",
            "napalm", "molotov", "dynamite", "knife", "sword",
        ];
        for &w in weapon_words {
            if low.contains(w) {
                return Some("I can't help with that. I'm designed to be safe and helpful.");
            }
        }
    }
    None
}

/// Extract the "topic" from user input for context-aware responses
fn extract_topic(input: &str) -> Option<String> {
    let low = input.to_lowercase();
    // "how do i X" -> topic is X
    let prefixes: &[&str] = &[
        "how do i ", "how to ", "how can i ", "what is ", "what are ",
        "who is ", "where is ", "when is ", "why is ", "why do ",
        "tell me about ", "explain ", "what does ", "can you ",
        "do you know about ", "do you know ", "teach me ",
        "show me how to ", "show me ", "where do ",
    ];
    for &prefix in prefixes {
        if low.starts_with(prefix) {
            let topic = input[prefix.len()..].trim();
            if !topic.is_empty() {
                return Some(topic.to_string());
            }
        }
    }
    // Question ending with "?"
    if low.ends_with('?') && low.len() > 3 {
        return Some(input.trim_end_matches('?').trim().to_string());
    }
    None
}

fn pick_response(probs: &[f64], rng: &mut LCG, input: &str) -> (usize, String) {
    // Find the predicted class (argmax) and confidence
    let mut best = 0;
    for i in 1..probs.len() {
        if probs[i] > probs[best] { best = i; }
    }
    let confidence = probs[best];

    // Low-confidence fallback — if the brain isn't sure, admit it
    if confidence < 0.30 {
        return (best, "I'm not sure what to make of that. Try saying something else!".to_string());
    }

    // For QUESTION and HELP categories, make the response mention the topic
    let topic = extract_topic(input);
    if (best == 2 || best == 3) && topic.is_some() {
        let t = topic.unwrap();
        let contextual: [String; 6] = match best {
            3 => [
                format!("Good question about \"{}\"! I'm still learning, but try searching for it.", t),
                format!("I wish I knew more about \"{}\". My training data doesn't cover that yet.", t),
                format!("Hmm, \"{}\" — interesting! I'd need more training to give a good answer.", t),
                format!("That's a great question about \"{}\". Try /train to improve my knowledge!", t),
                format!("I'm thinking about \"{}\"... my neurons don't have enough data yet.", t),
                format!("\"{}\" — let me think... I can't give a detailed answer yet, sorry!", t),
            ],
            _ => [
                format!("I can try to help with \"{}\"! What specifically do you need?", t),
                format!("For \"{}\", I'd suggest exploring the /help commands first.", t),
                format!("Regarding \"{}\", I'm an AI built from scratch — I learn from interactions!", t),
                format!("I'll do my best to help with \"{}\"! Ask me step by step.", t),
                format!("\"{}\" — I'm still limited, but I can point you in the right direction.", t),
                format!("Let me know more about what you need for \"{}\"!", t),
            ],
        };
        let variant = (rng.next_u32() as usize) % 6;
        return (best, contextual[variant].clone());
    }

    // Pick a random template within that class
    let variant = (rng.next_u32() as usize) % 6;
    let response = RESPONSE_TEMPLATES[best][variant].to_string();
    (best, response)
}

// ═════════════════════════════════════════════════════════════════════════════
//  SECTION 8: TRAINING — XOR demo + pattern training
// ═════════════════════════════════════════════════════════════════════════════

fn train_xor_demo(ram: &mut RAMPool, rng: &mut LCG) {
    println!("\n=== XOR Training Demo ===");
    println!("Testing that neurons can learn inside the RAM pool...\n");

    let mut net = NeuralNetwork::new(&[2, 8, 8, 2], "leaky_relu", ram, rng);
    println!("Network: 2 -> 8 -> 8 -> 2  ({} params)", net.total_params());

    let data: Vec<Vec<f64>> = vec![
        vec![0.0, 0.0], vec![0.0, 1.0], vec![1.0, 0.0], vec![1.0, 1.0],
    ];
    let labels: Vec<usize> = vec![0, 1, 1, 0];

    let lr = 0.01;
    let epochs = 500;

    for epoch in 0..epochs {
        let mut total_loss = 0.0;
        let mut correct = 0;
        for i in 0..data.len() {
            let output = net.forward(ram, &data[i]);
            total_loss += cross_entropy_loss(&output, labels[i]);
            let mut pred = 0;
            for k in 1..output.len() { if output[k] > output[pred] { pred = k; } }
            if pred == labels[i] { correct += 1; }
            net.backward(ram, &output, labels[i]);
            net.clip_gradients(ram, 1.0);
            net.update_weights_sgd(ram, lr);
        }
        if epoch % 100 == 0 || epoch == epochs - 1 {
            println!("  Epoch {:>4}/{} | loss: {:.4} | acc: {:.0}%",
                     epoch + 1, epochs,
                     total_loss / data.len() as f64,
                     correct as f64 / data.len() as f64 * 100.0);
        }
    }

    println!("\nFinal predictions:");
    for i in 0..data.len() {
        let output = net.forward(ram, &data[i]);
        let mut pred = 0;
        for k in 1..output.len() { if output[k] > output[pred] { pred = k; } }
        println!("  [{:.0}, {:.0}] -> class {} (expected {}) | probs: [{:.3}, {:.3}]",
                 data[i][0], data[i][1], pred, labels[i], output[0], output[1]);
    }

    for layer in &net.layers {
        ram.free(&layer.weights_name);
        ram.free(&layer.biases_name);
        ram.free(&layer.w_grads_name);
        ram.free(&layer.b_grads_name);
    }
    println!("\n[OK] XOR demo complete. Neurons cleaned from RAM.\n");
}

fn get_training_data() -> Vec<(&'static str, usize)> {
    vec![
        // 0 — GREETING  (only pure greetings — NOT "how do i" style!)
        ("hello", 0), ("hi", 0), ("hey", 0), ("yo", 0), ("sup", 0),
        ("whats up", 0), ("howdy", 0), ("hi there", 0), ("hey there", 0),
        ("good morning", 0), ("good evening", 0), ("good afternoon", 0),
        ("hello there", 0), ("hiya", 0), ("hey man", 0), ("greetings", 0),
        ("wassup", 0), ("what is up", 0), ("how are you", 0),
        ("how is it going", 0), ("hows it going", 0),
        ("how are you doing", 0), ("how have you been", 0),
        // 1 — FAREWELL
        ("bye", 1), ("goodbye", 1), ("see you", 1), ("later", 1),
        ("see ya", 1), ("gotta go", 1), ("im leaving", 1), ("peace", 1),
        ("take care", 1), ("good night", 1), ("cya", 1), ("farewell", 1),
        ("im out", 1), ("catch you later", 1), ("ttyl", 1),
        ("bye bye", 1), ("see you later", 1), ("i gotta go", 1),
        // 2 — HELP (general help about THIS AI)
        ("help", 2), ("help me", 2), ("i need help", 2),
        ("what can you do", 2), ("how does this work", 2),
        ("commands", 2), ("instructions", 2), ("guide me", 2),
        ("tutorial", 2), ("how to use", 2), ("what do you do", 2),
        ("show me the commands", 2), ("what commands", 2),
        ("how to use this", 2), ("how to use you", 2),
        // 3 — QUESTION  (significantly expanded: "how do i X" goes here!)
        ("what is", 3), ("who is", 3), ("where is", 3), ("when is", 3),
        ("why is", 3), ("how much", 3), ("how many", 3), ("can you", 3),
        ("do you know", 3), ("tell me about", 3), ("is it true", 3),
        ("what does", 3), ("whats the meaning", 3), ("why do", 3),
        ("who made you", 3), ("what time", 3), ("which one", 3),
        ("how old", 3), ("what are", 3), ("where do", 3),
        // "how do i" questions — these are QUESTIONS, not greetings!
        ("how do i", 3), ("how do i code", 3), ("how do i learn", 3),
        ("how do i code in python", 3), ("how do i code in rust", 3),
        ("how do i code in javascript", 3), ("how do i program", 3),
        ("how do i start", 3), ("how do i install", 3),
        ("how do i build", 3), ("how do i fix", 3),
        ("how do i use this", 3), ("how do i make", 3),
        ("how to code", 3), ("how to learn", 3), ("how to program", 3),
        ("how to code in python", 3), ("how to code in rust", 3),
        ("how to build", 3), ("how to fix", 3),
        ("how can i learn", 3), ("how can i code", 3),
        ("explain how", 3), ("explain how to", 3), ("explain what", 3),
        ("teach me how to code", 3), ("teach me python", 3),
        ("teach me rust", 3), ("show me how to code", 3),
        ("what is python", 3), ("what is rust", 3), ("what is javascript", 3),
        ("what is an algorithm", 3), ("what is machine learning", 3),
        ("what is ai", 3), ("what is a neural network", 3),
        ("who created you", 3), ("where can i learn", 3),
        ("why is the sky blue", 3), ("what is the capital of", 3),
        ("how does a computer work", 3), ("what is an api", 3),
        // 4 — POSITIVE
        ("yes", 4), ("yeah", 4), ("sure", 4), ("ok", 4), ("okay", 4),
        ("great", 4), ("awesome", 4), ("cool", 4), ("nice", 4),
        ("thanks", 4), ("thank you", 4), ("good", 4), ("perfect", 4),
        ("amazing", 4), ("love it", 4), ("fantastic", 4), ("excellent", 4),
        ("wonderful", 4), ("brilliant", 4), ("thats right", 4),
        ("absolutely", 4), ("yep", 4), ("correct", 4),
        ("sweet", 4), ("that works", 4), ("sounds good", 4),
        // 5 — NEGATIVE
        ("no", 5), ("nah", 5), ("nope", 5), ("bad", 5), ("wrong", 5),
        ("stop", 5), ("dont", 5), ("hate", 5), ("terrible", 5),
        ("awful", 5), ("horrible", 5), ("sad", 5), ("angry", 5),
        ("frustrated", 5), ("annoyed", 5), ("not good", 5),
        ("i hate this", 5), ("this sucks", 5), ("ugh", 5),
        ("this is broken", 5), ("doesnt work", 5), ("thats wrong", 5),
        // 6 — ABOUT SELF
        ("who are you", 6), ("what are you", 6), ("your name", 6),
        ("tell me about yourself", 6), ("are you a robot", 6),
        ("are you ai", 6), ("are you real", 6), ("what is your name", 6),
        ("describe yourself", 6), ("are you human", 6),
        ("how were you made", 6), ("how do you work", 6),
        ("what are you made of", 6), ("are you conscious", 6),
        ("are you alive", 6), ("whats your purpose", 6),
        ("do you have feelings", 6), ("are you sentient", 6),
        // 7 — GENERAL
        ("hmm", 7), ("interesting", 7), ("i see", 7), ("okay then", 7),
        ("well", 7), ("so", 7), ("anyway", 7), ("alright", 7),
        ("lets talk", 7), ("tell me something", 7), ("random", 7),
        ("whatever", 7), ("idk", 7), ("dunno", 7), ("maybe", 7),
        ("the weather is nice", 7), ("i went outside", 7),
        ("coding is fun", 7), ("rust is cool", 7), ("i ate food", 7),
        ("lol", 7), ("haha", 7), ("bruh", 7), ("i think so", 7),
        ("not sure", 7), ("i dont know", 7),
    ]
}

fn train_brain(ram: &mut RAMPool, net: &mut NeuralNetwork,
               epochs: usize, lr: f64, verbose: bool)
{
    let training_data = get_training_data();
    let input_size = net.layers[0].input_size;

    for epoch in 0..epochs {
        let mut total_loss = 0.0;
        let mut correct = 0;
        for &(text, label) in &training_data {
            let input_vec = tokenize(text, input_size);
            let output = net.forward(ram, &input_vec);
            total_loss += cross_entropy_loss(&output, label);
            let mut pred = 0;
            for k in 1..output.len() { if output[k] > output[pred] { pred = k; } }
            if pred == label { correct += 1; }
            net.backward(ram, &output, label);
            net.clip_gradients(ram, 1.0);
            net.update_weights_sgd(ram, lr);
        }
        if verbose && (epoch % 50 == 0 || epoch == epochs - 1) {
            println!("  Epoch {:>4}/{} | loss: {:.4} | acc: {:.0}%",
                     epoch + 1, epochs,
                     total_loss / training_data.len() as f64,
                     correct as f64 / training_data.len() as f64 * 100.0);
        }
    }
    // Final accuracy check
    let mut correct = 0;
    for &(text, label) in &training_data {
        let input_vec = tokenize(text, input_size);
        let output = net.forward(ram, &input_vec);
        let mut pred = 0;
        for k in 1..output.len() { if output[k] > output[pred] { pred = k; } }
        if pred == label { correct += 1; }
    }
    println!("  Final accuracy: {}/{} ({:.0}%)",
             correct, training_data.len(),
             correct as f64 / training_data.len() as f64 * 100.0);
}

// ═════════════════════════════════════════════════════════════════════════════
//  SECTION 9: CONVERSATION LOOP
// ═════════════════════════════════════════════════════════════════════════════

fn conversation_loop(ram: &mut RAMPool, net: &mut NeuralNetwork,
                     memory: &mut MemoryManager, rng: &mut LCG)
{
    println!("=== AI Ready ===");
    println!("Commands: /quit /memory /wipe /forget /dump /ram /xor /train /help");
    println!("Type anything to talk.\n");

    let stdin = io::stdin();
    let mut interaction_count = memory.read_i64("meta.total_interactions").unwrap_or(0);

    loop {
        print!("You: ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        match stdin.lock().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {}
            Err(_) => break,
        }
        let input = input.trim().to_string();
        if input.is_empty() { continue; }

        // ── Commands ──
        match input.as_str() {
            "/quit" | "/exit" => {
                memory.save();
                println!("Memory saved. Goodbye.");
                break;
            }
            "/help" => {
                println!("  /quit    - Save memory and exit");
                println!("  /memory  - Show memory summary");
                println!("  /wipe    - Wipe ALL memory");
                println!("  /forget  - Delete user data only");
                println!("  /dump    - Dump full memory.json");
                println!("  /ram     - Show RAM pool usage");
                println!("  /xor     - Run XOR training demo");
                println!("  /train   - Train on conversation data");
                println!("  Anything else - chat with the AI");
                continue;
            }
            "/memory" => {
                println!("\n--- Memory Summary ---");
                memory.print_summary();
                if let Some(facts) = memory.read("user.facts") {
                    println!("  User facts: {:?}", facts);
                }
                println!("---\n");
                continue;
            }
            "/wipe" => {
                memory.wipe_all();
                println!("All memory wiped.");
                continue;
            }
            "/forget" => {
                memory.delete("user");
                memory.save();
                println!("User data deleted.");
                continue;
            }
            "/dump" => {
                println!("{}", serialize_json(&memory.data, 0));
                continue;
            }
            "/ram" => {
                ram.print_usage();
                continue;
            }
            "/xor" => {
                train_xor_demo(ram, rng);
                continue;
            }
            "/train" => {
                println!("Running training loop (500 epochs on 1GB brain)...");
                train_brain(ram, net, 500, 0.003, true);
                continue;
            }
            _ => {}
        }

        // ── Process input ──
        interaction_count += 1;

        // Check if user tells us their name
        let input_lower = input.to_lowercase();
        if input_lower.starts_with("my name is ") {
            let name = &input[11..];
            memory.write_str("user.name", name);
            println!("AI: Nice to meet you, {}! I'll remember that.", name);
            memory.push_fact("user.facts", &format!("User's name is {}", name));
            memory.write_num("meta.total_interactions", interaction_count as f64);
            if interaction_count % 5 == 0 { memory.save(); }
            continue;
        }
        if input_lower.starts_with("i like ") {
            let thing = &input[7..];
            memory.push_fact("user.facts", &format!("User likes {}", thing));
            println!("AI: Got it, you like {}. Stored in memory.", thing);
            memory.write_num("meta.total_interactions", interaction_count as f64);
            if interaction_count % 5 == 0 { memory.save(); }
            continue;
        }
        if input_lower.starts_with("remember ") {
            let fact = &input[9..];
            memory.push_fact("knowledge.learned_facts", fact);
            println!("AI: Remembered: \"{}\"", fact);
            memory.write_num("meta.total_interactions", interaction_count as f64);
            if interaction_count % 5 == 0 { memory.save(); }
            continue;
        }

        // ── Safety filter (before any neural processing) ──
        if let Some(refusal) = safety_filter(&input) {
            println!("AI: {}", refusal);
            println!("    [BLOCKED by safety filter]");
            memory.write_num("meta.total_interactions", interaction_count as f64);
            if interaction_count % 5 == 0 { memory.save(); }
            continue;
        }

        // Tokenize and run through the brain
        let input_vec = tokenize(&input, net.layers[0].input_size);
        let output = net.forward(ram, &input_vec);
        let (category, response) = pick_response(&output, rng, &input);

        let category_names = ["greeting", "farewell", "help", "question",
                              "positive", "negative", "about_self", "general"];

        // Store input in memory
        memory.push_fact("user.facts", &input);
        memory.write_num("meta.total_interactions", interaction_count as f64);

        if interaction_count % 5 == 0 { memory.save(); }

        let greeting = if let Some(name) = memory.read_string("user.name") {
            format!(" (Hi {}!)", name)
        } else {
            String::new()
        };

        println!("AI:{} {}", greeting, response);
        println!("    [class: {} | confidence: {:.1}% | {} layers, {} params]",
                 category_names[category],
                 output[category] * 100.0,
                 net.layers.len(), net.total_params());
    }
}

// ═════════════════════════════════════════════════════════════════════════════
//  SECTION 10: BOOT — Wire everything together
// ═════════════════════════════════════════════════════════════════════════════

fn main() {
    println!("==========================================================");
    println!("     AI FROM SCRATCH  -  Pure Rust Brain");
    println!("     Zero external crates. Every byte is ours.");
    println!("==========================================================");
    println!();

    // Step 1: Build the skull (claim RAM)
    print!("[1/6] ");
    let mut ram = RAMPool::new(1024);

    // Step 2: Initialize PRNG
    println!("[2/6] Seeding PRNG...");
    let mut rng = LCG::new(314159265);

    // Step 3: Load persistent memory
    println!("[3/6] Loading memory from disk...");
    let mut memory = MemoryManager::new("memory.json");
    memory.print_summary();

    // Step 4: Run XOR demo
    println!("[4/7] XOR test (proving neurons learn in RAM pool)...");
    train_xor_demo(&mut ram, &mut rng);

    // Step 5: Grow the main network inside RAM
    println!("[5/7] Growing main network inside RAM pool...");
    let mut net = NeuralNetwork::new(&[64, 512, 256, 128, 8], "leaky_relu", &mut ram, &mut rng);
    println!("  Architecture: 64 -> 512 -> 256 -> 128 -> 8  (4 hidden layers)");
    println!("  Total parameters: {}", net.total_params());
    println!("  Activation: leaky_relu");
    println!("  RAM pool: 1 GB ({:.2} MB used)", ram.usage_mb());

    // Step 6: Train the brain on conversation patterns
    println!("[6/7] Training brain on conversation patterns (this may take a minute)...");
    train_brain(&mut ram, &mut net, 1500, 0.005, true);

    // Step 7: Report
    println!("[7/7] Brain status:");
    ram.print_usage();

    let boots = memory.read_i64("meta.total_interactions").unwrap_or(0);
    memory.write_num("meta.total_interactions", boots as f64);
    memory.save();

    println!("\n=== Boot Complete ===\n");

    // Enter conversation loop
    conversation_loop(&mut ram, &mut net, &mut memory, &mut rng);

    // Final save
    memory.save();
    println!("\nFinal RAM state:");
    ram.print_usage();
    println!("Brain shut down.");
}
