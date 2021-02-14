use ggez::*;
use ggez::conf::{WindowMode,Conf};
use ggez::event::EventHandler;
use wwtm_project::source::Assets;
use wwtm_project::question_list::QuestionList;
use ggez::graphics::{DrawParam,DrawMode,Rect,Color};
use ggez::mint::Point2;


use std::env;
use std::path;

struct GameState{
  game_over:bool,
  score:u32,
  assets:Assets,
  questions:QuestionList,
  screen_width: f32,
  screen_height: f32,
}


impl GameState{
  fn new(ctx: &mut Context, conf :&Conf)-> GameResult<GameState> {
    let _assets = Assets::new(ctx)?;
    let _questions = QuestionList::new();
    let gs = Self
      {
        game_over:false,
        score:0,
        assets:_assets,
        questions:_questions,
        screen_height:conf.window_mode.height,
        screen_width:conf.window_mode.width
      };

    Ok(gs)
  }

}

impl EventHandler for GameState {
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    /*
    Алгоритъм:
      зарежда един въпрос заедно с 4 отговора.Зарежда също кой отговор е правилен.
      Който се от отговорите се натисне, се чака 4 секунди и се отчита дали е верен.
      Ако е верен,зарежда следващ въпрос и отговори,иначе спира играта. 




    */

    Ok(())
  }
  fn draw(&mut self, _ctx: &mut Context) -> GameResult {
    //draws the background
    let default = graphics::DrawParam::new();
    graphics::draw(_ctx,&self.assets.background,default)?;

    //draws the question placeholder
    let question_rect = graphics::Mesh::new_rectangle(
      _ctx,
       DrawMode::fill(),
     Rect::new(60.0,300.0,680.0,80.0),
     Color::new(0.0,0.0,40.0,0.95))?;
    graphics::draw(_ctx,&question_rect,DrawParam::default())?;

    //draws the first answer placeholder
    let question_rect = graphics::Mesh::new_rectangle(
      _ctx,
       DrawMode::fill(),
     Rect::new(60.0,400.0,330.0,40.0),
     Color::new(0.0,0.0,40.0,0.95))?;
    graphics::draw(_ctx,&question_rect,DrawParam::default())?;

    //draws the second answer placeholder
    let question_rect = graphics::Mesh::new_rectangle(
      _ctx,
       DrawMode::fill(),
     Rect::new(410.0,400.0,330.0,40.0),
     Color::new(0.0,0.0,40.0,0.95))?;
    graphics::draw(_ctx,&question_rect,DrawParam::default())?;

    //draws the third answer placeholder
    let question_rect = graphics::Mesh::new_rectangle(
      _ctx,
       DrawMode::fill(),
     Rect::new(60.0,450.0,330.0,40.0),
     Color::new(0.0,0.0,40.0,0.95))?;
    graphics::draw(_ctx,&question_rect,DrawParam::default())?;

    //draws the fourth answer placeholder
    let question_rect = graphics::Mesh::new_rectangle(
      _ctx,
       DrawMode::fill(),
     Rect::new(410.0,450.0,330.0,40.0),
     Color::new(0.0,0.0,40.0,0.95))?;
    graphics::draw(_ctx,&question_rect,DrawParam::default())?;

    graphics::present(_ctx)?;
    Ok(())
    
  } 
}


pub fn main() {
  let c = conf::Conf::new().window_mode(WindowMode {
    width: 800.0,
    height: 600.0,
    ..Default::default()
  });

  
  let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("project", "simeon")
    .conf(c.clone())
    .build()
    .unwrap();

  //  load the assets from resources
  if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
      let mut path = path::PathBuf::from(manifest_dir);
      path.push("resources");
      filesystem::mount( ctx, &path, true);
  }
  
  //load the state
  let state = &mut GameState::new(ctx, &c).unwrap(); 

  //run!
  event::run(ctx, event_loop, state).unwrap();
}



