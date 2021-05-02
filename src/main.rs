use ggez::*;
use ggez::conf::{WindowMode,Conf};
use ggez::event::{EventHandler,KeyCode,KeyMods};
use ggez::graphics::{DrawParam,DrawMode,Rect,Color};
use ggez::mint::{Point2,Vector2};
use ggez::audio::SoundSource;
use ggez::input::mouse;

use rand::Rng;
use rand::rngs::ThreadRng;

use wwtm_project::source::Assets;
use wwtm_project::question_list::QuestionList;
use wwtm_project::question::Question;

use std::env;
use std::path;

enum AnswerState{
  Marked,
  NotMarked,
  Correct,
  Wrong,
}


struct GameState{
  rng:ThreadRng,
  fifty_fifty_used:bool,
  help_public :bool,
  friend_call:bool,
  game_over:bool,
  player_resigned:bool,
  has_won:bool,
  next_question:bool,
  current_score:u32,
  assets:Assets,
  questions:QuestionList,
  answer_state:AnswerState,
  answer_marked:char,
  current_question:Question,
  current_question_index:usize, // helps finding question score
  saved_score:u32, //cap 500,2500,100000 or the current_score if the player decides to resign
  score_board:Vec<u32>,
  time_marked:f32,
  time_transition:f32,


}


impl GameState{
  fn new(ctx: &mut Context, _conf :&Conf)-> GameResult<GameState> {
    let _assets = Assets::new(ctx)?;
    let mut _questions = QuestionList::new();
    let  first_question =_questions.next().unwrap().clone();
    
    let gs = Self
      {
        rng: rand::thread_rng(),
        fifty_fifty_used:false,
        help_public:false,
        friend_call:false,
        game_over:false,
        player_resigned:false,
        has_won:false,
        next_question:true,
        current_score:0,
        assets:_assets,
        questions:_questions,
        answer_state:AnswerState::NotMarked,
        answer_marked:' ',
        current_question:first_question,
        current_question_index:0,
        saved_score:0,
        score_board: vec!(0,50,100,200,300,500,750,1000,1500,2000,2500,5000,10000,25000,50000,100000),
        time_marked:2.0,
        time_transition:0.5,
        
      };

    Ok(gs)
  }

  fn joker_50_50(&mut self){
    let first_removed = self.rng.gen_range(1..3);
    let mut second_removed = self.rng.gen_range(1..3);

    while first_removed == second_removed {
        second_removed = self.rng.gen_range(1..3);
    }

    if self.current_question.correct_answer == 'a' {
      if first_removed == 1 {
        self.current_question.a2_show = false;
      }
      else if first_removed == 2 {
        self.current_question.a3_show = false;
      }
      else {
        self.current_question.a4_show = false;
      }

      if second_removed == 1 {
        self.current_question.a2_show = false;
      }
      else if second_removed == 2 {
        self.current_question.a3_show = false;
      }
      else {
        self.current_question.a4_show = false;
      }

    }
    else if self.current_question.correct_answer == 'b'{
      if first_removed == 1 {
        self.current_question.a1_show = false;
      }
      else if first_removed == 2 {
        self.current_question.a3_show = false;
      }
      else {
        self.current_question.a4_show = false;
      }

      if second_removed == 1 {
        self.current_question.a1_show = false;
      }
      else if second_removed == 2 {
        self.current_question.a3_show = false;
      }
      else {
        self.current_question.a4_show = false;
      }
    }
    else if self.current_question.correct_answer == 'c'{
      if first_removed == 1 {
        self.current_question.a1_show = false;
      }
      else if first_removed == 2 {
        self.current_question.a2_show = false;
      }
      else {
        self.current_question.a4_show = false;
      }

      if second_removed == 1 {
        self.current_question.a1_show = false;
      }
      else if second_removed == 2 {
        self.current_question.a2_show = false;
      }
      else {
        self.current_question.a4_show = false;
      }
    }
    else if self.current_question.correct_answer == 'd'{
      if first_removed == 1 {
        self.current_question.a1_show = false;
      }
      else if first_removed == 2 {
        self.current_question.a2_show = false;
      }
      else {
        self.current_question.a3_show = false;
      }

      if second_removed == 1 {
        self.current_question.a1_show = false;
      }
      else if second_removed == 2 {
        self.current_question.a2_show = false;
      }
      else {
        self.current_question.a3_show = false;
      }
    }
    else{
      ()
    }
  }


}

impl EventHandler for GameState {

  fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods,_repeat:bool){
    match keycode {
      event::KeyCode::R =>self.player_resigned  = true,
      event::KeyCode::Escape => event::quit(ctx),
      _=> () 
    }
  }
  

  fn update(&mut self, ctx: &mut Context) -> GameResult {
    
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

    
    //the game won`t do anything if the mouse isn`t pressed
    while  mouse::button_pressed(ctx, mouse::MouseButton::Left) == false{
      return Ok(());
    }

    let mouse_pos = mouse::position(ctx);

    if  mouse_pos.x >= 60.0 && mouse_pos.x <=390.0 && mouse_pos.y >= 400.0 && mouse_pos.y <= 440.0{
      self.answer_state = AnswerState::Marked;
      self.answer_marked = 'a';
    }
    else if  mouse_pos.x >= 410.0 && mouse_pos.x <= 740.0 && mouse_pos.y >= 400.0 && mouse_pos.y <= 440.0 {
      self.answer_state = AnswerState::Marked;
      self.answer_marked = 'b';
    }
    else if  mouse_pos.x >= 60.0 && mouse_pos.x <= 390.0  && mouse_pos.y >= 450.0 && mouse_pos.y <= 490.0{
      self.answer_state = AnswerState::Marked;
      self.answer_marked = 'c';
    }
    else if  mouse_pos.x >= 410.0 && mouse_pos.x <= 740.0  && mouse_pos.y >= 450.0 && mouse_pos.y <= 490.0{
      self.answer_state = AnswerState::Marked;
      self.answer_marked = 'd';
    }
   

    //use joker 50/50
    if mouse_pos.x <= 30.0 && mouse_pos.y >= 180.0 && mouse_pos.y <= 210.0 && self.fifty_fifty_used == false {
      self.joker_50_50();
      self.fifty_fifty_used = true;
    }
    //use joker "Help from the public"
    if mouse_pos.x <= 30.0 && mouse_pos.y >= 215.0 && mouse_pos.y <= 245.0 && self.help_public == false {
      self.help_public = true;
    }
     //use joker "Call a friend"
    if mouse_pos.x <= 30.0 && mouse_pos.y >= 250.0 && mouse_pos.y <= 280.0 && self.friend_call == false {
      self.friend_call = true;
    }


    let desired_fps = 60;
    let second = 1.0/desired_fps as f32;  
    self.next_question = false;

    //change transtition time for questions 5 and 10
    if (self.current_question_index == 4 || self.current_question_index == 9) && self.time_marked >=2.0 {
      self.time_transition = 3.5;
    }

    
    //TODO: Премести кода за въпросите така,че да може да покрие случая при победа.Прекарай един тест да си припомниш.
    while timer::check_update_time(ctx, desired_fps) == true{
      //if an answer is marked, start substracting seconds
        if self.answer_marked == 'a' || self.answer_marked == 'b' || self.answer_marked == 'c' || self.answer_marked == 'd'  {
          self.time_marked -=second;
          if self.time_marked < 0.0 {
            //when the marked question is right
            if self.current_question.correct_answer == self.answer_marked{    
                //save the score after question 5,10 and play the saved_score sound effect
                match self.current_question_index{
                  4 =>{
                    self.saved_score = 500;
                    let _=self.assets.saved_score_sound.play();
                    self.assets.main_theme.pause();
                    self.answer_state = AnswerState::Correct;
                    self.time_transition -= second;
                    if self.time_transition <=0.0{
                      self.time_transition = 0.5;
                      self.time_marked = 2.0;
                      self.next_question = true;
                     // self.assets.saved_score_sound.stop();
                      let _ =self.assets.main_theme.play();
                    }
                    
                  },
                  9 =>{
                    self.saved_score = 2500;
                    let _=self.assets.saved_score_sound.play();
                    self.assets.main_theme.pause();
                    self.answer_state = AnswerState::Correct;
                    self.time_transition -= second;
                    if self.time_transition <= 0.0{
                      self.time_transition = 0.5;
                      self.time_marked = 2.0;
                      self.next_question = true;
                      //self.assets.saved_score_sound.stop();
                      let _ =self.assets.main_theme.play();
                    }
                    
                  },
                  14 =>{
                    self.saved_score = 100000;
                    self.next_question = true;
                  },
                  _ =>
                  {   
                    //play the win sound and wait for 2 seconds
                    let _ = self.assets.right_question_sound.play();
                    self.answer_state =  AnswerState::Correct;
                    self.time_transition -= second;
                    if self.time_transition <= 0.0{
                      self.time_transition = 0.5;
                      self.time_marked = 2.0;
                      self.next_question = true;
                    }
                  }
                }
                
                //check if next question exists
                if self.next_question == true {
                  let next_question = self.questions.next();
                  match next_question {
                    Some(_) => {
                      self.current_question = next_question.unwrap();
                      self.answer_marked = ' '; 
                      self.answer_state =  AnswerState::NotMarked;
                    
                    },
                    None =>{ //no more questions in the list so the player has won the game
                      self.has_won = true;
                      let _ = self.assets.win_sound.play();
                      self.assets.main_theme.stop();
                    }
                  }
                  if self.current_question_index < 15{ 
                    self.current_question_index +=1;
                  }
                  
                }
            }
            //when the answer is wrong
            else{
              self.answer_state =  AnswerState::Wrong;
              self.assets.main_theme.stop();
              let _= self.assets.wrong_question_sound.play();
              self.time_transition -= second;
              if self.time_transition <= 0.0{
                self.game_over = true;
              }    
            }
          }
          self.draw(ctx)?;
        }
        self.current_score =self.score_board[self.current_question_index];

    }
    Ok(())
  }


  fn draw(&mut self, ctx: &mut Context) -> GameResult {

    //the blue screen of celebration
    if self.has_won == true{
      let dark_blue = graphics::Color::from_rgb(26, 51, 77);
      graphics::clear(ctx, dark_blue);

      let text = graphics::Text::new(format!("Congratulations!You finished the game.\nTake your 100000 leva and go live your live"));
      graphics::draw(ctx,&text,DrawParam
        {dest:Point2{x: 200.0,y: 275.0},
        scale:Vector2{x:1.25,y:1.25},
        ..Default::default()})?;
      graphics::present(ctx)?;

      return Ok(());
    }


    //the blue screen of meh
    if self.game_over == true{
      let dark_blue = graphics::Color::from_rgb(26, 51, 77);
      graphics::clear(ctx, dark_blue);

      let text = graphics::Text::new(format!("Game over!\nYour score is {}",self.saved_score));
      graphics::draw(ctx,&text,DrawParam
        {dest:Point2{x: 325.0,y: 270.0},
          scale:Vector2{x:1.25,y:1.25},
        ..Default::default()})?;
      graphics::present(ctx)?;

      return Ok(());
    }


    //draws the background
    graphics::draw(ctx,&self.assets.background,DrawParam{..Default::default()})?;

    //draws a joker placeholder
    let joker_rect =  graphics::Mesh::new_rectangle(
      ctx,
       DrawMode::fill(),
     Rect::new(0.0,0.0,30.0,30.0),
     Color::new(0.0,0.0,40.0,0.95))?;
    // 50/50 joker
    if self.fifty_fifty_used == false {
      graphics::draw(ctx,&joker_rect,DrawParam{
        dest:Point2{x:00.0,y:180.0},
        ..Default::default()
      })?;
    }

    // help public joker
    if self.fifty_fifty_used == false {
      graphics::draw(ctx,&joker_rect,DrawParam{
        dest:Point2{x:00.0,y:215.0},
        ..Default::default()
      })?;
    }

    // call friend joker
    if self.fifty_fifty_used == false {
      graphics::draw(ctx,&joker_rect,DrawParam{
        dest:Point2{x:00.0,y:250.0},
        ..Default::default()
      })?;
    }

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

    //draw the answer placeholders
    graphics::draw(ctx,&answer_rect,DrawParam{
        dest:Point2{x:60.0,y:400.0},
        color:Color::from_rgb(0,0,40),
        ..Default::default()
      })?;

    graphics::draw(ctx,&answer_rect,DrawParam{
        dest:Point2{x:410.0,y:400.0},
        color:Color::from_rgb(0,0,40),
        ..Default::default()
      })?;

    graphics::draw(ctx,&answer_rect,DrawParam{
        dest:Point2{x:60.0,y:450.0},
        color:Color::from_rgb(0,0,40),
        ..Default::default()
      })?;

    graphics::draw(ctx,&answer_rect,DrawParam{
      dest:Point2{x:410.0,y:450.0},
      color:Color::from_rgb(0,0,40),
      ..Default::default()
    })?;

    let sign_a = graphics::Text::new("a)");
    graphics::draw(ctx, &sign_a,DrawParam{
      dest:Point2{x:70.0,y:410.0},
      
      ..Default::default()
    })?; 

    let sign_b = graphics::Text::new("b)");
    graphics::draw(ctx, &sign_b,DrawParam{
      dest:Point2{x:420.0,y:410.0},
      ..Default::default()
    })?; 

    let sign_c = graphics::Text::new("c)");
    graphics::draw(ctx, &sign_c,DrawParam{
      dest:Point2{x:70.0,y:460.0},
      ..Default::default()
    })?; 

    let sign_d = graphics::Text::new("d)");
    graphics::draw(ctx, &sign_d,DrawParam{
      dest:Point2{x:420.0,y:460.0},
      ..Default::default()
    })?; 


  //marking logic  
  if self.answer_marked == 'a'{
      match self.answer_state{
        AnswerState::Marked =>
          graphics::draw(ctx,&answer_rect,DrawParam{
            dest:Point2{x:60.0,y:400.0},
            color:Color::from_rgb(225,157,0),
            ..Default::default()
          })?,
        AnswerState::Correct =>
          graphics::draw(ctx,&answer_rect,DrawParam{
            dest:Point2{x:60.0,y:400.0},
            color:Color::from_rgb(0,157,0),
            ..Default::default()
          })?,
        AnswerState::Wrong =>
          graphics::draw(ctx,&answer_rect,DrawParam{
            dest:Point2{x:60.0,y:400.0},
            color:Color::from_rgb(225,0,0),
            ..Default::default()
          })?,
        _ => (),  
      }
  }
  else if self.answer_marked == 'b' {
    match self.answer_state{
      AnswerState::Marked =>
        graphics::draw(ctx,&answer_rect,DrawParam{
          dest:Point2{x:410.0,y:400.0},
          color:Color::from_rgb(225,157,0),
          ..Default::default()
        })?,
      AnswerState::Correct =>
        graphics::draw(ctx,&answer_rect,DrawParam{
          dest:Point2{x:410.0,y:400.0},
          color:Color::from_rgb(0,157,0),
          ..Default::default()
        })?,
      AnswerState::Wrong =>
        graphics::draw(ctx,&answer_rect,DrawParam{
          dest:Point2{x:410.0,y:400.0},
          color:Color::from_rgb(225,0,0),
          ..Default::default()
        })?,
      _ => (),
    }
  }
  else if self.answer_marked == 'c'{
    match self.answer_state{
      AnswerState::Marked =>
        graphics::draw(ctx,&answer_rect,DrawParam{
          dest:Point2{x:60.0,y:450.0},
          color:Color::from_rgb(225,157,0),
          ..Default::default()
        })?,
      AnswerState::Correct =>
        graphics::draw(ctx,&answer_rect,DrawParam{
          dest:Point2{x:60.0,y:450.0},
          color:Color::from_rgb(0,157,0),
          ..Default::default()
        })?,
      AnswerState::Wrong =>
        graphics::draw(ctx,&answer_rect,DrawParam{
          dest:Point2{x:60.0,y:450.0},
          color:Color::from_rgb(225,0,0),
          ..Default::default()
        })?,
      _ => (),  
    }
  }
  else{
    match self.answer_state{
      AnswerState::Marked =>
        graphics::draw(ctx,&answer_rect,DrawParam{
          dest:Point2{x:410.0,y:450.0},
          color:Color::from_rgb(225,157,0),
          ..Default::default()
        })?,
      AnswerState::Correct =>
        graphics::draw(ctx,&answer_rect,DrawParam{
          dest:Point2{x:410.0,y:450.0},
          color:Color::from_rgb(0,157,0),
          ..Default::default()
        })?,
      AnswerState::Wrong =>
        graphics::draw(ctx,&answer_rect,DrawParam{
          dest:Point2{x:410.0,y:450.0},
          color:Color::from_rgb(225,0,0),
          ..Default::default()
        })?,
      _ => (),  
    }
  }
   
     //draw a message showing saved scores
     if self.current_score == 500 && self.current_score == 2500{
      let message = graphics::Text::new(format!("You now have capped {} leva",self.current_score));
      graphics::draw(ctx,&message,DrawParam{dest:Point2{x: 175.0,y: 120.0},..Default::default()})?;
     
    }

    //draws the question
    self.current_question.draw(ctx)?;

    //draw the current score
    let text = graphics::Text::new(format!("Score: {}",self.current_score));
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

