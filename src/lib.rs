// src/lib.rs
//
// &        = Referenz/Borrow -> Daten ausleihen ohne Ownership zu übernehmen
// &mut     = mutable Referenz -> ausleihen UND verändern dürfen
// mut      = Variable ist veränderbar -> default immutable
// ::       = Pfad-Operator -> navigiert durch Module/Typen (. in Java)
// |x|      = Closure -> anonyme Funktion (wie Lambda)
// Option<T>= entweder Some(wert) oder None
// Result<T>= entweder Ok(wert) oder Err(fehler) -> kein try/catch
// &str     = geliehener String-Slice (kein Ownership)
// String   = owned String
// &[T]     = geliehener Slice -> Referenz auf eine Folge von T-Werten
// impl     = Methoden für einen Typ definieren (wie Klassen-Body in Java)
// trait    = Interface

// ---------- 1. BASICS ----------
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name) // format! = String zusammenbauen (wie f-string in Python)
}

pub fn sum(nums: &[i32]) -> i32 {
    let mut sum: i32 = 0; // mut = veränderbar, sonst kann sum nicht += werden
    for element in nums {
        sum += element;
    }
    sum // kein Semikolon -> Rückgabewert
}

pub fn flip(b: bool) -> bool {
    // TODO: return the opposite boolean
    !b
}

// ---------- 2. OWNERSHIP & BORROWING ----------
pub fn take_ownership(s: String) -> usize {
    // s: String (nicht &String) → Ownership wird in die Funktion gemoved
    // Aufrufer kann s danach nicht mehr verwenden
    s.len()
}

pub fn borrow_first_char(s: &str) -> Option<char> {
    // TODO: return the first char without taking ownership
    s.chars().next()
}

pub fn push_exclamation(s: &mut String) {
    // &mut = geliehen & veränderbar _> Änderung wirkt auf das Original beim Aufrufer
    s.push('!') // push = einzelnes char anhängen (push_str für &str)
}

// ---------- 3. STRUCTS, ENUMS, METHODS ----------
#[derive(Debug, Clone, Copy, PartialEq)] // Compiler generiert automatisch: Ausgabe, Kopieren, Vergleichen
pub struct Point {
    pub x: f64,
    pub y: f64,
}

use std::fmt; // fmt Modul importieren -> "import java.util...."

impl fmt::Display for Point { // Display = was bei {} in println! ausgegeben wird (Debug wäre {:?})
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { // Pflichtmethode von Display
        write!(f, "({}, {})", self.x, self.y) // wie println! aber in f schreiben statt Konsole, kein ; = Rückgabewert
    }
}

impl Point { // impl = Methoden für Point definieren
    pub fn distance_to(&self, other: &Point) -> f64 {
        // &self = Methode borgt sich die Instanz (kein Ownership) "this" in Java
        ((other.x - self.x) * (other.x - self.x) + (other.y - self.y) * (other.y - self.y)).sqrt()
    }

    pub fn origin() -> Self {
        // kein &self -> statische Methode
        Self { x: 0.0, y: 0.0 }
    }
}
// #[derive(...)] lässt den Compiler automatisch Traits implementieren:
// Debug    → erlaubt {:?} in println! für Debugging
// Clone    → erlaubt .clone() für explizite Kopien
// Copy     → Typ wird automatisch kopiert statt gemoved (für kleine Typen)
// PartialEq→ erlaubt == und != Vergleiche
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape { // enum kann Daten enthalten
    Circle { center: Point, radius: f64 },
    Rect { top_left: Point, w: f64, h: f64 },
    Triangle { a: Point, b: Point, c: Point },
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self { // match -> switch case
            Shape::Circle { center, radius } => (radius * radius) * std::f64::consts::PI,
            Shape::Rect { top_left, w, h } => w * h,
            Shape::Triangle { a, b, c } => {
                ((a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y)) / 2.0).abs()
            }
        }
    }
}

// ---------- 4. TRAITS & GENERICS ----------
pub trait Plottable { // trait = Interface
    fn x(&self) -> f64;
    fn y(&self) -> f64;
}

impl Plottable for Point { // "Point implementiert Plottable" (implements in Java)
    fn x(&self) -> f64 { self.x }
    fn y(&self) -> f64 { self.y }
}

impl Plottable for (f64, f64) {
    fn x(&self) -> f64 { self.0 } // .0 und .1 = Felder eines Tuples
    fn y(&self) -> f64 { self.1 }
}

pub fn furthest_from_origin<T: Plottable>(items: &[T]) -> Option<&T> {
    // <T: Plottable> = Generic mit Constraint — T muss Plottable implementieren (wie <T extends Plottable> in Java)
    let mut max_dist: f64 = 0.0;
    let mut result: Option<&T> = None;
    for item in items {
        let distance = item.x() * item.x() + item.y() * item.y();
        if distance > max_dist {
            max_dist = distance;
            result = Some(item); // item in Option einwickeln
        }
    }
    result // Some(&T) oder None wenn items leer war
}

// ---------- 5. ERRORS & OPTION/RESULT ----------
pub fn parse_port(s: &str) -> Result<u16, String> {
    // Result<OK-Typ, Fehler-Typ> -> kein try/catch, Fehler sind normale Werte
    s.parse::<u16>().map_err(|e| e.to_string()) // map_err = Fehlertyp umwandeln, nur bei Err aufgerufen
}

// ---------- 6. ITERATORS & CLOSURES ----------
pub fn even_squares(n: u32) -> Vec<u32> {
    (0..=n).filter(|x| x % 2 == 0).map(|x| x * x).collect()
}

// ---------- 7. USING A CRATE (rand) ----------
use rand::Rng; // Rng = Trait, muss im Scope sein damit .gen_range() funktioniert

pub fn roll_dice(sides: u8) -> u8 {
    rand::thread_rng().gen_range(1..=sides) // Zufallszahlengenerator für den aktuellen Thread
}