use ggez::audio;
use ggez::graphics;
use ggez::{Context, GameResult};

pub struct Assets{
    pub background:graphics::Image, //заден фон на екрана
    pub main_theme: audio::Source, // главната музика по време на игра
    pub saved_score_sound:audio::Source,//
    pub win_sound:audio::Source,//
    pub right_question_sound:audio::Source,
    pub resign_sound :audio::Source,
    pub wrong_question_sound:audio::Source,

}

impl Assets{
   pub fn new(ctx:&mut Context)->GameResult<Assets>{
        let background = graphics::Image::new(ctx,"/background1.jpeg")?;
        let main_theme = audio::Source::new(ctx,"/main theme.mp3")?;
        let saved_score_sound = audio::Source::new(ctx,"/saved score sound.mp3")?;
        let win_sound = audio::Source::new(ctx,"/win sound.mp3")?;
        let right_question_sound = audio::Source::new(ctx, "/right question sound.mp3")?;
        let resign_sound = audio::Source::new(ctx,"/resign sound.mp3")?;
        let wrong_question_sound = audio::Source::new(ctx,"/loss sound.mp3")?;


        Ok(Assets{background,main_theme,saved_score_sound,
            win_sound,right_question_sound,resign_sound,wrong_question_sound})
    }
}