use ggez::audio;
use ggez::graphics;
use ggez::{Context, GameResult};

pub struct Assets{
    pub background:graphics::Image, //заден фон на екрана
    pub main_theme: audio::Source, // главната музика по време на игра
    pub saved_score_sound:audio::Source,//
    pub win_sound:audio::Source,//
}

impl Assets{
   pub fn new(ctx:&mut Context)->GameResult<Assets>{
        let background = graphics::Image::new(ctx,"/background1.jpeg")?;
        let main_theme = audio::Source::new(ctx,"/main theme.mp3")?;
        let saved_score_sound = audio::Source::new(ctx,"/saved score sound.mp3")?;
        let win_sound = audio::Source::new(ctx,"/win sound.mp3")?;


        Ok(Assets{background,main_theme,saved_score_sound,win_sound})
    }
}