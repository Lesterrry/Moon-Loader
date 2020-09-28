/***************************
COPYRIGHT FETCH DEVELOPMENT,
2020
LESTERRRY AUTHORSHIP
***************************/
use std::time::Instant;

mod moon_loader;
use moon_loader::*;

fn main(){
	let mut loader = MoonLoader::new(MoonLoaderVariant::Moon, true);

	let mut start = Instant::now();
	for _i in 0..20 {
		loader.draw();
		//println!("{:?}", start.elapsed());
		while start.elapsed().as_millis() < 200 {
		}
		//println!("{:?}", start.elapsed().as_millis());
		start = Instant::now();
	}
}