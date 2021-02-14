use ggez::audio;
use ggez::graphics;
use ggez::{Context, GameResult};

pub struct Assets{
    pub background:graphics::Image, //заден фон на екрана
    pub main_theme: audio::Source, // главната музика по време на игра
}

impl Assets{
   pub fn new(ctx:&mut Context)->GameResult<Assets>{
        let background = graphics::Image::new(ctx,"/background.jpeg")?;
        let main_theme = audio::Source::new(ctx,"/main theme.mp3")?;


        Ok(Assets{background,main_theme})
    }
}