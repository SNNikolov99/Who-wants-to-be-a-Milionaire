use ggez::*;
use ggez::conf::{WindowMode,Conf};
use ggez::event::{EventHandler,KeyCode,KeyMods};
use ggez::graphics::{DrawParam,DrawMode,Rect,Color};
use ggez::mint::{Point2,Vector2};
use ggez::audio::SoundSource;
use ggez::input::{keyboard::is_key_pressed,mouse};

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
  game_over:bool,
  player_resigned:bool,
  has_won:bool,
  current_score:u32,
  assets:Assets,
  questions:QuestionList,
  answer_state:AnswerState,
  answer_marked:char,
  current_question:Question,
  current_question_index:usize, // helps finding question score
  saved_score:u32, //cap 500,2500,100000 or the current_score if the player decides to resign
  score_board:Vec<u32>,
  //time_marked:f32,


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
        current_score:0,
        assets:_assets,
        questions:_questions,
        answer_state:AnswerState::NotMarked,
        answer_marked:' ',
        current_question:first_question,
        current_question_index:0,
        saved_score:0,
        score_board: vec!(0,50,100,200,300,500,750,1000,1500,2000,2500,5000,10000,25000,50000,100000),
        
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
    else {
      return Ok(());
    }

    self.draw(ctx)?;

    //when the marked question is right
    if self.current_question.correct_answer == self.answer_marked{
      
        self.current_score =self.score_board[self.current_question_index + 1];
        //save the score after question 5,10 and play the saved_score sound effect
        match self.current_question_index{
          4 =>{
            ggez::timer::sleep(std::time::Duration::new(2,0));
            self.saved_score = 500;
            self.draw(ctx)?;
            //draw the change of colour
            self.answer_state = AnswerState::Correct;
            self.draw(ctx)?;
            self.play_sc_sound();
            
          },
          9 =>{
            ggez::timer::sleep(std::time::Duration::new(2,0));
            self.saved_score = 2500;
            //draw the change of colour
            self.answer_state =  AnswerState::Correct;
            self.draw(ctx)?;
            self.play_sc_sound();
          },
          14 =>self.saved_score = 100000,
          _ =>
          {   
            //play the win sound and wait for 2 seconds
            ggez::timer::sleep(std::time::Duration::new(2,0));
            //draw the change of colour
            self.answer_state =  AnswerState::Correct;
            self.draw(ctx)?;
            let _ = self.assets.right_question_sound.play();
          }
        }
        
      

        //check if next question exists
        ggez::timer::sleep(std::time::Duration::new(1,0));
        let next_question = self.questions.next();
        match next_question {
          Some(_) => {
            self.current_question = next_question.unwrap();
            self.answer_marked = '0'; // null the marked question
            self.answer_state =  AnswerState::NotMarked;//return the colour to blue
           
          },
          None =>{
            //no more questions in the list so the player has won the game
            self.has_won = true;
            let _ = self.assets.win_sound.play();
            self.assets.main_theme.stop();
          }
        }
        self.current_question_index +=1;
        
    }
    //when the answer is wrong
    else{
      //draw the change of colour
      self.answer_state =  AnswerState::Wrong;
      ggez::timer::sleep(std::time::Duration::new(2,0));
      self.draw(ctx)?;
      self.assets.main_theme.stop();
      let _= self.assets.wrong_question_sound.play();
      ggez::timer::sleep(std::time::Duration::new(1,500));
      self.game_over = true;
       
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

/*
fn update(&mut self, ctx: &mut Context) -> GameResult {
        //draw the context before doing something
        
        const DESIRED_FPS:u32 = 60;
        let seconds = 1.0/(DESIRED_FPS as f32);
    
        //what happens if game is over
        if self.game_over == true{
          return Ok(());
        }
    
        //what happens if player resigns
        if self.player_resigned == true{
          self.saved_score = self.current_score;
           //make the exit more smooth
          while timer::check_update_time(ctx,DESIRED_FPS){
            self.answer_timer -=seconds;
            
            if self.answer_timer <= 0.0 {
               self.assets.main_theme.stop();
               let _=self.assets.resign_sound.play();
               self.game_over = true;
            }
          }
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
        if self.current_question.correct_answer == self.answer_marked{
           
            self.should_continue = false;
            self.current_score =*score_board_iter.nth(self.current_question_index).unwrap();
            //save the score after question 5,10 and play the saved_score sound effect
            while timer::check_update_time(ctx,DESIRED_FPS) == true{
              self.answer_timer -=seconds;
              self.draw(ctx)?;

              if self.answer_timer<= 0.0 {
                //self.answer_state = AnswerState::Correct;  
                match self.current_question_index{
                    4 =>{
                      self.saved_score = 500;
                            self.play_sc_sound();
                            self.answer_timer = 2.0;
                            self.should_continue = true;
                    },
                    9 =>{
                            self.saved_score = 2500;
                            self.play_sc_sound();
                            self.answer_timer = 2.0;
                            self.should_continue = true;
                    },
                    14 =>self.saved_score = 100000,

                    _ =>{ 
                        self.transition_timer -=seconds;
                        if self.transition_timer <= 0.0 {
                            self.answer_state = AnswerState::Correct;  
                            self.should_continue = true;
                            let _ = self.assets.right_question_sound.play();
                            
                            self.answer_timer = 2.0;
                            self.transition_timer = 1.0;
                          }
                        //self.draw(ctx)?;
                        }
                }
              }
              }
    
            //check if next question exists
            if self.should_continue == true {
              let next_question = self.questions.next();
              match next_question {
                Some(_) => {
                  self.current_question = next_question.unwrap();
                  self.answer_marked = '0'; // null the marked question
                  self.answer_state =  AnswerState::NotMarked;//return the colour to blue
                
                },
                None =>{
                  //no more questions in the list so the player has won the game
                  self.has_won = true;
                  let _ = self.assets.win_sound.play();
                  self.assets.main_theme.stop();
                }
              }
              self.current_question_index +=1;
            }
            
        }
        //when the answer is wrong
        else{
          //draw the change of colour
          self.draw(ctx)?;
          while timer::check_update_time(ctx,DESIRED_FPS){
            self.answer_timer -=seconds;
    
            if self.answer_timer <= 0.0 {
              self.answer_state =  AnswerState::Wrong;
              self.assets.main_theme.stop();
              let _= self.assets.wrong_question_sound.play();
              self.transition_timer -=seconds;
              if self.transition_timer <= 0.0{
                self.answer_timer = 2.0;
                self.game_over = true;
              }
            }
          }
           
        }
    
        Ok(())
      }
*/



/*
fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.draw(ctx)?;  
        //what happens if game is over
        if self.game_over == true{
          return Ok(());
        }
    
        //what happens if player resigns
        if self.player_resigned == true{
          self.saved_score = self.current_score;
           //make the exit more smooth
               self.assets.main_theme.stop();
               let _=self.assets.resign_sound.play();
               self.game_over = true;
            
          
          return Ok(());
        }
    

        
        //the game won t do anything if one of these keys isn t pressed
        while !is_key_pressed(ctx,KeyCode::A) && !is_key_pressed(ctx,KeyCode::B) &&
              !is_key_pressed(ctx,KeyCode::C) && !is_key_pressed(ctx,KeyCode::D)
              {
                return Ok(());
              }
        
        const DESIRED_FPS:u32 = 60;
        let seconds = 1.0/(DESIRED_FPS as f32);  
        self.draw(ctx)?;  
        //answer logic
        while timer::check_update_time(ctx, DESIRED_FPS) == true{
          
            if self.current_question.correct_answer == self.answer_marked{
              self.answer_timer -=seconds;
              //save the score after question 5,10 and play the saved_score sound effect
                if self.answer_timer <= 0.0 {
                  match self.current_question_index{
                      4 =>{
                              self.saved_score = 500;
                              //self.play_sc_sound(ctx);
                              self.answer_timer = 3.0;
                              self.should_continue = true;
                              
                      },
                      9 =>{
                              self.saved_score = 2500;
                              //self.play_sc_sound(ctx);
                              self.answer_timer = 3.0;
                              self.should_continue = true;
                              
                      },
                      14 => self.saved_score = 100000,
                      _ =>{
                          self.answer_state = AnswerState::Correct; 
                          self.transition_timer -=seconds;
                          let _ = self.assets.right_question_sound.play();
                          if self.transition_timer <= 0.0 {
                              self.should_continue = true; 
                              self.answer_timer = 3.0;
                              self.transition_timer = 2.0;
                          }
                      }
                  }
                }
                self.current_score =self.score_board[self.current_question_index + 1]; 
                
            }
            //when the answer is wrong
            else{
             
                if self.answer_timer <= 0.0 {
                  self.answer_state =  AnswerState::Wrong;
                  self.assets.main_theme.stop();
                  let _= self.assets.wrong_question_sound.play();
                  self.transition_timer -=seconds;
                  if self.transition_timer <= 0.0{
                    self.game_over = true;
                  }
                }
             }
             
        } 

        if self.should_continue == true {
          let next_question = self.questions.next();
          match next_question {
            Some(_) => {
              self.current_question = next_question.unwrap();
              self.answer_marked = ' '; // null the marked question
              self.answer_state =  AnswerState::NotMarked;//return the colour to blue
              self.should_continue = false;
            
            },
            None =>{
              //no more questions in the list so the player has won the game
              self.has_won = true;
              let _ = self.assets.win_sound.play();
              self.assets.main_theme.stop();
            }
          }
          self.current_question_index +=1;
        }

        self.draw(ctx)?;  
        Ok(())
    }
*/


