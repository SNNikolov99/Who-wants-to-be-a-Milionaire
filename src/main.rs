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
  right_answer:bool,
  current_score:u32,
  assets:Assets,
  questions:QuestionList,
  question_marked:char,
  current_question:Question,
  current_question_index:usize, // helps finding question score
  saved_score:u32, //cap 500,2500,100000 or the current_score if the player decides to resign

}


impl GameState{
  fn new(ctx: &mut Context, _conf :&Conf)-> GameResult<GameState> {
    let _assets = Assets::new(ctx)?;
    let mut _questions = QuestionList::new();
    let  first_question =_questions.next().unwrap().clone();
    
    let gs = Self
      {
        game_over:false,
        player_resigned:false,
        has_won:false,
        right_answer:false,
        current_score:0,
        assets:_assets,
        questions:_questions,
        question_marked:'0',
        current_question:first_question,
        current_question_index:0,
        saved_score:0,
        
      };

    Ok(gs)
  }

  //play the saved score sound
  fn play_sc_sound(& mut self){
      let _=self.assets.saved_score_sound.play();
      if self.assets.main_theme.playing() == true{
            self.assets.main_theme.pause();
            ggez::timer::sleep(std::time::Duration::new(6,0));
            self.assets.saved_score_sound.stop();
            let _ =self.assets.main_theme.play();
          }
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
  


    /*
    Алгоритъм:
      зарежда един въпрос заедно с 4 отговора.Зарежда също кой отговор е правилен.
      Който се от отговорите се натисне, се чака 4 секунди и се отчита дали е верен.
      Ако е верен,зарежда следващ въпрос и отговори,иначе спира играта. 
    
      */
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    //draw the context before doing something
    self.draw(ctx)?;
    
    //what happens if game is over
    if self.game_over == true{
      return Ok(());
    }

    //what happens if player resigns
    if self.player_resigned == true{
      self.saved_score = self.current_score;
      self.game_over = true;
      ggez::timer::sleep(std::time::Duration::new(2,0));//make the exit more smooth
      self.assets.main_theme.stop();
      let _=self.assets.resign_sound.play();
      return Ok(());
    }

    //question I gives score I
    let score_board = vec!(0,50,100,200,500,750,1000,1500,2000,2500,5000,10000,20000,50000,100000);
    let mut score_board_iter = score_board.iter();
    
    //the game won t do anything if one of these keys isn t pressed
    while !is_key_pressed(ctx,KeyCode::A) && !is_key_pressed(ctx,KeyCode::B) &&
          !is_key_pressed(ctx,KeyCode::C) && !is_key_pressed(ctx,KeyCode::D)
          {
            return Ok(());
          }
    
    //when the marked question is right
    if self.current_question.correct_answer == self.question_marked{
        self.current_score =*score_board_iter.nth(self.current_question_index).unwrap();
        //save the score after question 5,10 and play the saved_score sound effect
        match self.current_question_index{
          4 =>{
            self.saved_score = 500;
            self.play_sc_sound();
            
          },
          9 =>{
            self.saved_score = 2500;
            self.play_sc_sound();
          },
          14 =>self.saved_score = 100000,
          _ => () //do nothing
        }
        //play the win sound and wait for 2 seconds
        if self.current_question_index != 4 && self.current_question_index != 9{
          ggez::timer::sleep(std::time::Duration::new(2,0));
          self.right_answer = true;
          let _ = self.assets.right_question_sound.play();
        }

        //check if next question exists
        ggez::timer::sleep(std::time::Duration::new(1,0));
        let next_question = self.questions.next();
        match next_question {
          Some(_) => {
            self.current_question = next_question.unwrap();
            self.question_marked = '0'; // null the marked question
            self.right_answer = false;
          },
          None =>{
            self.has_won = true;
            let _ = self.assets.win_sound.play();
            self.assets.main_theme.stop();
          }
        }
        self.current_question_index +=1;
        
    }
    //when the answer is wrong
    else{
      ggez::timer::sleep(std::time::Duration::new(2,0));
      self.game_over = true;
      self.assets.main_theme.stop();
      let _= self.assets.wrong_question_sound.play();
      
    }

    Ok(())
  }


  fn draw(&mut self, ctx: &mut Context) -> GameResult {

    //the blue screen of celebration
    if self.has_won == true{
      let dark_blue = graphics::Color::from_rgb(26, 51, 77);
      graphics::clear(ctx, dark_blue);

      let text = graphics::Text::new(format!("Congratulations!You finished the game. \n Take your 100000 leva and go live your live"));
      graphics::draw(ctx,&text,DrawParam{dest:Point2{x: 200.0,y: 300.0},..Default::default()})?;
      graphics::present(ctx)?;

      return Ok(());
    }


    //the blue screen of meh
    if self.game_over == true{
      let dark_blue = graphics::Color::from_rgb(26, 51, 77);
      graphics::clear(ctx, dark_blue);

      let text = graphics::Text::new(format!("Game over!\n Your score is {}",self.saved_score));
      graphics::draw(ctx,&text,DrawParam{dest:Point2{x: 350.0,y: 300.0},..Default::default()})?;
      graphics::present(ctx)?;

      return Ok(());
    }


    //draws the background
    let default = graphics::DrawParam::new();
    graphics::draw(ctx,&self.assets.background,default)?;

    //draws the question placeholder
    let question_rect = graphics::Mesh::new_rectangle(
      ctx,
       DrawMode::fill(),
     Rect::new(60.0,300.0,680.0,80.0),
     Color::new(0.0,0.0,40.0,0.95))?;
    graphics::draw(ctx,&question_rect,DrawParam::default())?;

    //draws an answer placeholder
    let answer_rect = graphics::Mesh::new_rectangle(
      ctx,
       DrawMode::fill(),
      Rect::new(0.0,0.0,330.0,40.0),
      Color::new(255.0,255.0,255.0,0.95))?;


    //draws the first answer placeholder
    if self.question_marked =='a'{
        graphics::draw(ctx,&answer_rect,DrawParam{
          dest:Point2{x:60.0,y:400.0},
          color:Color::from_rgb(255,157,0),
          ..Default::default()
          })?;
      }
    else{
        graphics::draw(ctx,&answer_rect,DrawParam{
          dest:Point2{x:60.0,y:400.0},
          color:Color::from_rgb(0,0,40),
          ..Default::default()
        })?;
      }
  

    let sign_a = graphics::Text::new("a)");
    graphics::draw(ctx, &sign_a,DrawParam{
      dest:Point2{x:70.0,y:410.0},
      
      ..Default::default()
    })?; 

    //draws the second answer placeholder
    if self.question_marked =='b'{
      graphics::draw(ctx,&answer_rect,DrawParam{
        dest:Point2{x:410.0,y:400.0},
        color:Color::from_rgb(255,157,0),
        ..Default::default()
      })?;
    }
    else{
      graphics::draw(ctx,&answer_rect,DrawParam{
        dest:Point2{x:410.0,y:400.0},
        color:Color::from_rgb(0,0,40),
        ..Default::default()
      })?;
    }
    
  
    let sign_b = graphics::Text::new("b)");
    graphics::draw(ctx, &sign_b,DrawParam{
      dest:Point2{x:420.0,y:410.0},
      ..Default::default()
    })?; 

     //draws the third answer placeholder
     if self.question_marked =='c'{
      graphics::draw(ctx,&answer_rect,DrawParam{
        dest:Point2{x:60.0,y:450.0},
        color:Color::from_rgb(255,157,0),
        ..Default::default()
      })?;
     }
     else{
      graphics::draw(ctx,&answer_rect,DrawParam{
        dest:Point2{x:60.0,y:450.0},
        color:Color::from_rgb(0,0,40),
        ..Default::default()
      })?;
    }

    let sign_c = graphics::Text::new("c)");
    graphics::draw(ctx, &sign_c,DrawParam{
      dest:Point2{x:70.0,y:460.0},
      ..Default::default()
    })?; 

     //draws the fourth answer placeholder
    if self.question_marked =='d'{
        graphics::draw(ctx,&answer_rect,DrawParam{
          dest:Point2{x:410.0,y:450.0},
          color:Color::from_rgb(225,157,0),
          ..Default::default()
        })?;
        /*
        if self.right_answer == true{
          graphics::draw(ctx,&answer_rect,DrawParam{
            dest:Point2{x:410.0,y:450.0},
            color:Color::from_rgb(0,157,0),
            ..Default::default()
          })?;
        }
        else{
          graphics::draw(ctx,&answer_rect,DrawParam{
            dest:Point2{x:410.0,y:450.0},
            color:Color::from_rgb(225,0,0),
            ..Default::default()
          })?;
        }
        */
    }
    else{
      graphics::draw(ctx,&answer_rect,DrawParam{
        dest:Point2{x:410.0,y:450.0},
        color:Color::from_rgb(0,0,40),
        ..Default::default()
      })?;

    }
    let sign_d = graphics::Text::new("d)");
    graphics::draw(ctx, &sign_d,DrawParam{
      dest:Point2{x:420.0,y:460.0},
      ..Default::default()
    })?; 

    //draw the question
    self.current_question.draw(ctx)?;

    //draw the current score
    let text = graphics::Text::new(format!("Score :{}",self.current_score));
    graphics::draw(ctx,&text,DrawParam{dest:Point2{x: 675.0,y: 20.0},..Default::default()})?;

    graphics::present(ctx)?;
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
    let _= state.assets.main_theme.play();
 
  //run!
  event::run(ctx, event_loop, state).unwrap();
}



