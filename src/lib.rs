/***************************
COPYRIGHT FETCH DEVELOPMENT,
2020
LESTERRRY AUTHORSHIP
***************************/
use std::io::{self, Write};

const MOONS: [char; 8] = ['🌕', '🌖', '🌗', '🌘', '🌑', '🌒', '🌓', '🌔'];
const EARTHS: [char; 3] = ['🌎', '🌍', '🌏'];
const CLOCKS: [char; 24] = ['🕐', '🕜', '🕑', '🕝', '🕒', '🕞', '🕓', '🕟', '🕔', '🕠', '🕕', '🕡', '🕖', '🕢', '🕗', '🕣', '🕘', '🕤', '🕙', '🕥', '🕚', '🕦', '🕛', '🕧'];

#[allow(dead_code)]
pub enum MoonLoaderVariant{
	Moon,
	Earth,
	Clock,
}

#[allow(dead_code)]
pub struct MoonLoader{
	variant: MoonLoaderVariant,
	brackets: bool,
	progression: usize,

}

#[allow(dead_code)]
impl MoonLoader{
	pub fn new(variant: MoonLoaderVariant, brackets: bool) -> Self{
		MoonLoader{
			variant,
			brackets,
			progression: 0
		}
	}
	pub fn draw(&mut self) {
		let list: &[char];
		match self.variant {
			MoonLoaderVariant::Moon => {list = &MOONS},
			MoonLoaderVariant::Earth => {list = &EARTHS},
			MoonLoaderVariant::Clock => {list = &CLOCKS},
		}
		//print!("{}", self.progression);
		print!("\x1b[100H{}{}{} ", if self.brackets{"["} else {""}, list[self.progression], if self.brackets{"]"} else {""});
		io::stdout().flush().unwrap();
		if self.progression < list.len() - 1 { self.progression += 1 } else { self.progression = 0 }
	}
}
