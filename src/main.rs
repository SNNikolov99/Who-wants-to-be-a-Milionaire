use ggez::*;
use ggez::conf::{WindowMode,Conf};
use ggez::event::{EventHandler,KeyCode,KeyMods};
use ggez::graphics::{DrawParam,DrawMode,Rect,Color};
use ggez::mint::Point2;
use ggez::audio::SoundSource;
use ggez::input::keyboard::is_key_pressed;

use wwtm_project::source::Assets;
use wwtm_project::question_list::QuestionList;
use wwtm_project::question::Question;

use std::env;
use std::path;

struct GameState{
  game_over:bool,
  player_resigned:bool,
  has_won:bool,
  current_score:u32,
  assets:Assets,
  questions:QuestionList,
  question_marked:char,
  current_question:Question,
  current_question_index:usize, // helps finding question score
  saved_score:u32, //cap 500,2500,100000 or the current_score if the player decides to resign

}


impl GameState{
  fn new(ctx: &mut Context, conf :&Conf)-> GameResult<GameState> {
    let _assets = Assets::new(ctx)?;
    let mut _questions = QuestionList::new();
    let  first_question =_questions.next().unwrap().clone();
    
    let gs = Self
      {
        game_over:false,
        player_resigned:false,
        has_won:false,
        current_score:0,
        assets:_assets,
        questions:_questions,
        question_marked:'d',
        current_question:first_question,
        current_question_index:0,
        saved_score:0,
        
      };

    Ok(gs)
  }

}

impl EventHandler for GameState {

  fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods,_repeat:bool){
    match keycode {
      event::KeyCode::A =>self.question_marked = 'a',
      event::KeyCode::B =>self.question_marked = 'b',
      event::KeyCode::C =>self.question_marked = 'c',
      event::KeyCode::D =>self.question_marked = 'd',
      event::KeyCode::R =>self.player_resigned  = true,
      event::KeyCode::Escape => event::quit(ctx),
      _=> () 
    }
  }



  fn update(&mut self, ctx: &mut Context) -> GameResult {
    /*
    Алгоритъм:
      зарежда един въпрос заедно с 4 отговора.Зарежда също кой отговор е правилен.
      Който се от отговорите се натисне, се чака 4 секунди и се отчита дали е верен.
      Ако е верен,зарежда следващ въпрос и отговори,иначе спира играта. 
    
      */


    if self.game_over == true{
      return Ok(());
    }

  
    if self.player_resigned == true{
      self.saved_score = self.current_score;
      self.game_over = true;
      ggez::timer::sleep(std::time::Duration::new(2,0));//make the exit more smooth
      return Ok(());
    }
    
    let score_board = vec!(0,50,100,200,500,750,1000,1500,2000,2500,5000,10000,20000,50000,100000);//question I gives score I
    let mut score_board_iter = score_board.iter();
    
    //the game won t do anything if one of these keys isn t pressed
    while !is_key_pressed(ctx,KeyCode::A) && !is_key_pressed(ctx,KeyCode::B) &&
          !is_key_pressed(ctx,KeyCode::C) && !is_key_pressed(ctx,KeyCode::D){
            return Ok(());
          }
    
    if self.current_question.correct_answer == self.question_marked{
        self.current_score =*score_board_iter.nth(self.current_question_index).unwrap();
        //save the score after question 5,10 and 15
        match self.current_question_index{
          4 =>self.saved_score = 500,
          9 =>self.saved_score = 2500,
          14 =>self.saved_score = 100000,
          _ => () //do nothing
        }

        ggez::timer::sleep(std::time::Duration::new(2,0));
        //check if next question exists
        let next_question = self.questions.next();
        match next_question {
          Some(_) => self.current_question = next_question.unwrap(),
          None =>self.has_won = true,
        }
        self.current_question_index +=1;
        
    }
    else{
      ggez::timer::sleep(std::time::Duration::new(1,0));
      self.game_over = true;
    }

    Ok(())
  }


  fn draw(&mut self, _ctx: &mut Context) -> GameResult {

    //the blue screen of celebration
    if self.has_won == true{
      let dark_blue = graphics::Color::from_rgb(26, 51, 77);
      graphics::clear(_ctx, dark_blue);

      let text = graphics::Text::new(format!("Congratulations!You finished the game. \n Take your 100000 leva and go live your live"));
      graphics::draw(_ctx,&text,DrawParam{dest:Point2{x: 200.0,y: 300.0},..Default::default()})?;
      graphics::present(_ctx)?;

      return Ok(());
    }

    //the blue screen of meh
    if self.game_over == true{
      let dark_blue = graphics::Color::from_rgb(26, 51, 77);
      graphics::clear(_ctx, dark_blue);

      let text = graphics::Text::new(format!("Game over!\n Your score is {}",self.saved_score));
      graphics::draw(_ctx,&text,DrawParam{dest:Point2{x: 400.0,y: 300.0},..Default::default()})?;
      graphics::present(_ctx)?;

      return Ok(());
    }


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

    // answer placeholder
    let answer_rect = graphics::Mesh::new_rectangle(
      _ctx,
       DrawMode::fill(),
      Rect::new(0.0,0.0,330.0,40.0),
      Color::new(0.0,0.0,40.0,0.95))?;


    //draws the first answer placeholder
    graphics::draw(_ctx,&answer_rect,DrawParam{
      dest:Point2{x:60.0,y:400.0},
      ..Default::default()
    })?;

    let sign_a = graphics::Text::new("a)");
    graphics::draw(_ctx, &sign_a,DrawParam{
      dest:Point2{x:70.0,y:410.0},
      ..Default::default()
    })?; 

    //draws the second answer placeholder
    graphics::draw(_ctx,&answer_rect,DrawParam{
      dest:Point2{x:410.0,y:400.0},
      ..Default::default()
    })?;

    let sign_b = graphics::Text::new("b)");
    graphics::draw(_ctx, &sign_b,DrawParam{
      dest:Point2{x:420.0,y:410.0},
      ..Default::default()
    })?; 

     //draws the third answer placeholder
    graphics::draw(_ctx,&answer_rect,DrawParam{
      dest:Point2{x:60.0,y:450.0},
      ..Default::default()
    })?;

    let sign_c = graphics::Text::new("c)");
    graphics::draw(_ctx, &sign_c,DrawParam{
      dest:Point2{x:70.0,y:460.0},
      ..Default::default()
    })?; 

     //draws the fourth answer placeholder
    graphics::draw(_ctx,&answer_rect,DrawParam{
      dest:Point2{x:410.0,y:450.0},
      ..Default::default()
    })?;

    let sign_d = graphics::Text::new("d)");
    graphics::draw(_ctx, &sign_d,DrawParam{
      dest:Point2{x:420.0,y:460.0},
      ..Default::default()
    })?; 

    //draw the question
    self.current_question.draw(_ctx)?;

    //draw the current score
    let text = graphics::Text::new(format!("Score :{}",self.current_score));
    graphics::draw(_ctx,&text,DrawParam{dest:Point2{x: 675.0,y: 20.0},..Default::default()})?;

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

  //play the main theme
  let mut main_theme = audio::Source::new(ctx,"/main theme.mp3").unwrap();
  let _ = main_theme.play();

  //run!
  event::run(ctx, event_loop, state).unwrap();
}



