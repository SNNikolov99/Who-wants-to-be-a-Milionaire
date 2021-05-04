use ggez::*;
use ggez::conf::{WindowMode,Conf};
use ggez::event::{EventHandler,KeyCode,KeyMods};
use ggez::graphics::{DrawParam,DrawMode,Rect,Color};
use ggez::mint::{Point2,Vector2};
use ggez::audio::SoundSource;
use ggez::input::mouse;

use rand::Rng;
use rand::rngs::ThreadRng;
use rand::prelude::*;
use rand::distributions::WeightedIndex;

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
  help_public_index:usize,
  answer_public:(u32,u32,u32,u32),
  friend_call:bool,
  friend_call_index:usize, // on which question the "Call a friend joker" is used,so it cannot be shown on other questions
  game_over:bool,
  player_resigned:bool,
  has_won:bool,
  next_question:bool,
  current_score:u32,
  assets:Assets,
  questions:QuestionList,
  answer_state:AnswerState,
  answer_marked:char,
  answer_friend:char,
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
        help_public_index:0,
        answer_public:(0,0,0,0),
        friend_call:false,
        friend_call_index:0,
        game_over:false,
        player_resigned:false,
        has_won:false,
        next_question:true,
        current_score:0,
        assets:_assets,
        questions:_questions,
        answer_state:AnswerState::NotMarked,
        answer_marked:' ',
        answer_friend:' ',
        current_question:first_question,
        current_question_index:0,
        saved_score:0,
        score_board: vec!(0,50,100,200,300,500,750,1000,1500,2000,2500,5000,10000,25000,50000,100000),
        time_marked:2.0,
        time_transition:0.9,
        
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


  fn help_public(&mut self) -> (u32,u32,u32,u32){
    let mut a_supporters = 0;
    let mut b_supporters = 0;
    let mut c_supporters = 0;
    let mut d_supporters = 0;

    for _i in 1..100 {
      if self.friend_call() == 'a' {
        a_supporters +=1;
      }
      if self.friend_call() == 'b' {
        b_supporters +=1;
      }
      if self.friend_call() == 'c' {
        c_supporters +=1;
      }
      if self.friend_call() == 'd' {
        d_supporters +=1;
      }
    }
    (a_supporters,b_supporters,c_supporters,d_supporters)
  }

  fn friend_call(&mut self) -> char {
    let mut prob = 1.0; // probabiity for right answer
    let choices = ['a', 'b', 'c','d'];
    let mut weights = [1.0,1.0,1.0,1.0];

    if self.current_question_index > 0 && self.current_question_index <=4 {
      prob = 1.0;
    }
    else if self.current_question_index >=5 && self.current_question_index <=6 {
      prob = 0.90;
    }
    else if self.current_question_index >=7 && self.current_question_index <=8 {
      prob = 0.65;
    }
    else if self.current_question_index >=9 && self.current_question_index <=10{
      prob = 0.50;
    }
    else if self.current_question_index == 11 {
      prob = 0.35;
    }
    else if self.current_question_index == 12 {
      prob = 0.25;
    }
    else if self.current_question_index == 13 {
      prob = 0.15;
    }
    else if self.current_question_index == 14{
      prob = 0.05;
    }
    //calculate the weight for the reight answer
    if self.current_question.correct_answer == 'a' {
      weights = [prob,(1.0-prob)/3.0, (1.0-prob)/3.0, (1.0-prob)/3.0];
    }
    if self.current_question.correct_answer == 'b' {
      weights = [(1.0-prob)/3.0 ,prob, (1.0-prob)/3.0, (1.0-prob)/3.0];
    }
    if self.current_question.correct_answer == 'c' {
      weights = [(1.0-prob)/3.0,(1.0-prob)/3.0, prob, (1.0-prob)/3.0];
    }
    if self.current_question.correct_answer == 'd' {
      weights = [(1.0-prob)/3.0,(1.0-prob)/3.0, (1.0-prob)/3.0, prob];
    }
    let dist = WeightedIndex::new(&weights).unwrap(); 
    choices[dist.sample(&mut self.rng)]
  }

  /*
  fn update_markings(&mut self,ctx:&mut Context) -> GameResult{
    //draws an answer placeholder
    let answer_rect = graphics::Mesh::new_rectangle(
      ctx,
       DrawMode::fill(),
      Rect::new(0.0,0.0,330.0,40.0),
      Color::new(255.0,255.0,255.0,0.95))?;

    let mut answer_coord = Point2{x:0.0,y:0.0};
    if self.answer_marked == 'a'{
      answer_coord = Point2{x:60.0,y:400.0};
    }
    else if self.answer_marked == 'b' {
      answer_coord = Point2{x:410.0,y:400.0};
    }
    else if self.answer_marked == 'c' {
      answer_coord = Point2{x:60.0,y:450.0};
    }
    else if self.answer_marked == 'd'{
      answer_coord = Point2{x:410.0,y:450.0};
    }
    
    //decide which colour to use based on AnswerState
    let mut colour = Color::from_rgb(0,0,0);
    match self.answer_state{
      AnswerState::Marked =>{
        colour = Color::from_rgb(225,157,0);
      },
      AnswerState::Correct =>{
        colour = Color::from_rgb(0,157,0);
      },
      AnswerState::Wrong =>{
        colour = Color::from_rgb(225,0,0);
      },
      _ => (),  
    }
    
    graphics::draw(ctx,&answer_rect,DrawParam{
      dest:answer_coord,
      color:colour,
      ..Default::default()
    })?;
    graphics::present(ctx)?;
    Ok(())
  }
  */
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
      //ggez::timer::sleep(std::time::Duration::new(2,0));//make the exit more smooth
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
    if mouse_pos.x <= 38.0 && mouse_pos.y >= 180.0 && mouse_pos.y <= 210.0 && self.fifty_fifty_used == false {
      self.joker_50_50();
      self.fifty_fifty_used = true;
    }
    //use joker "Help from the public"
    if mouse_pos.x <= 38.0 && mouse_pos.y >= 215.0 && mouse_pos.y <= 245.0 && self.help_public == false {
      self.help_public = true;
      self.answer_public = self.help_public();
      self.help_public_index = self.current_question_index;
    }
     //use joker "Call a friend"
    if mouse_pos.x <= 38.0 && mouse_pos.y >= 250.0 && mouse_pos.y <= 280.0 && self.friend_call == false {
      self.friend_call = true;
      self.answer_friend = self.friend_call();
      self.friend_call_index = self.current_question_index;
      
    }


    let desired_fps = 18;
    let second = 1.0/desired_fps as f32;  
    self.next_question = false;

    //change transtition time for questions 5 and 10
    if (self.current_question_index == 4 || self.current_question_index == 9) && self.time_marked >=2.0 {
      self.time_transition = 5.5;
    }

    
    while timer::check_update_time(ctx, desired_fps) == true{
      //self.update_markings(ctx)?;
      self.draw(ctx)?;
      //if an answer is marked, start substracting seconds
        if self.answer_marked == 'a' || self.answer_marked == 'b' || self.answer_marked == 'c' || self.answer_marked == 'd'  {
          self.time_marked -=second;
          if self.time_marked <= 0.0 {
            //when the marked question is right
            if self.current_question.correct_answer == self.answer_marked{    
                //save the score after question 5,10 and play the saved_score sound effect
                match self.current_question_index{
                  4 =>{
                    self.saved_score = 500;
                    self.assets.main_theme.pause();
                    let _=self.assets.saved_score_sound.play();
                    self.answer_state = AnswerState::Correct;
                    self.time_transition -= second;
                    if self.time_transition <=0.0{
                      self.time_transition = 0.9;
                      self.time_marked = 2.0;
                      self.next_question = true;
                      self.assets.saved_score_sound.stop();
                      self.assets.main_theme.resume();
                    }
                    
                  },
                  9 =>{
                    self.saved_score = 2500;
                    self.assets.main_theme.pause();
                    let _=self.assets.saved_score_sound.play();
                    self.answer_state = AnswerState::Correct;
                    self.time_transition -= second;
                    if self.time_transition <= 0.0{
                      self.time_transition = 0.9;
                      self.time_marked = 2.0;
                      self.next_question = true;
                      self.assets.saved_score_sound.pause();
                      let _ =self.assets.main_theme.play();
                    }
                    
                  },
                  14 =>{
                    self.saved_score = 100000;
                    self.next_question = true;
                  },
                  _ =>
                  {   
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
                    None =>{ 
                      //no more questions in the list so the player has won the game
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
   
    // 50/50 joker
    if self.fifty_fifty_used == false {
      graphics::draw(ctx,&self.assets.joker_50_50,DrawParam{
        dest:Point2{x:0.0,y:180.0},
        scale:Vector2{x:0.1143,y:0.0886},
        ..Default::default()
      })?;
    }

    // help from public joker
    if self.help_public == false {
      graphics::draw(ctx,&self.assets.joker_help_public,DrawParam{
        dest:Point2{x:0.0,y:215.0},
        scale:Vector2{x:0.1143,y:0.0886},
        ..Default::default()
      })?;
    }
    else if self.current_question_index == self.help_public_index {
      let answer_a = graphics::Text::new(format!("People who support A : {} % ",self.answer_public.0));
      graphics::draw(ctx,&answer_a,DrawParam{dest:Point2{x: 175.0,y: 125.0},..Default::default()})?;
      let answer_b = graphics::Text::new(format!("People who support B : {} % ",self.answer_public.1));
      graphics::draw(ctx,&answer_b,DrawParam{dest:Point2{x: 175.0,y: 140.0},..Default::default()})?;
      let answer_c = graphics::Text::new(format!("People who support C : {} % ",self.answer_public.2));
      graphics::draw(ctx,&answer_c,DrawParam{dest:Point2{x: 175.0,y: 155.0},..Default::default()})?;
      let answer_d = graphics::Text::new(format!("People who support D : {} % ",self.answer_public.3));
      graphics::draw(ctx,&answer_d,DrawParam{dest:Point2{x: 175.0,y: 170.0},..Default::default()})?;
    } 

    // call a friend joker
      if self.friend_call == false  {
        graphics::draw(ctx,&self.assets.joker_friend_call,DrawParam{
          dest:Point2{x:0.0,y:250.0},
          scale:Vector2{x:0.1143,y:0.0886},
          ..Default::default()
        })?;
      }
      else if self.current_question_index == self.friend_call_index {
        let message = graphics::Text::new(format!("I think the answer is : {} ",self.answer_friend));
        graphics::draw(ctx,&message,DrawParam{dest:Point2{x: 175.0,y: 120.0},..Default::default()})?;
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
    let mut answer_coord = Point2{x:0.0,y:0.0};
    if self.answer_marked == 'a'{
      answer_coord = Point2{x:60.0,y:400.0};
    }
    else if self.answer_marked == 'b' {
      answer_coord = Point2{x:410.0,y:400.0};
    }
    else if self.answer_marked == 'c' {
      answer_coord = Point2{x:60.0,y:450.0};
    }
    else if self.answer_marked == 'd'{
      answer_coord = Point2{x:410.0,y:450.0};
    }

    
    let mut colour = Color::from_rgb(0,0,0);
    match self.answer_state{
      AnswerState::Marked =>{
        colour = Color::from_rgb(225,157,0);
      },
      AnswerState::Correct =>{
        colour = Color::from_rgb(0,157,0);
      },
      AnswerState::Wrong =>{
        colour = Color::from_rgb(225,0,0);
      },
      _ => (),  
    }
    
    graphics::draw(ctx,&answer_rect,DrawParam{
      dest:answer_coord,
      color:colour,
      ..Default::default()
    })?;

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