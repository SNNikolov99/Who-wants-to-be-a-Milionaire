use ggez::*;
use ggez::graphics::{Text,Align,DrawParam};
use ggez::mint::Point2;

/*
Въпрос с 4 отговора и стойност,която показва верния
*/
#[derive(Clone)]
pub struct Question{
    pub question:String,
    pub answer_1:String,
    pub answer_2:String,
    pub answer_3:String,
    pub answer_4:String,
    pub correct_answer:char,//a,b,c,d
    pub a1_show :bool,
    pub a2_show :bool,
    pub a3_show :bool,
    pub a4_show :bool,
}

impl Question{
    //initialise new question
    pub fn new(qustion_info:String)->Self{
        let mut str_iter = qustion_info.split('/').map(|x| x.trim());


        Self{
            question: str_iter.next().unwrap().to_string(),
            answer_1: str_iter.next().unwrap().to_string(),
            answer_2: str_iter.next().unwrap().to_string(),
            answer_3: str_iter.next().unwrap().to_string(),
            answer_4: str_iter.next().unwrap().to_string(),
            correct_answer:str_iter.next().unwrap().to_string().remove(0),
            a1_show :true,
            a2_show :true,
            a3_show :true,
            a4_show :true,
            
        }
    }

    //draw and update should be added
    pub fn draw(&mut self,_ctx:&mut Context) -> GameResult<()>{
        

        if self.a1_show == true {
          let mut dr_answer1 = Text::new(self.answer_1.as_str());
          dr_answer1.set_bounds(Point2{x:310.0,y:30.0}, Align::Center );
          //draws the first answer 
          graphics::draw(_ctx,&dr_answer1,DrawParam{
            dest:Point2{x:60.0,y:410.0},
            ..Default::default()
          })?;
        }

        if self.a2_show == true {
          let mut dr_answer2 = Text::new(self.answer_2.as_str());
          dr_answer2.set_bounds(Point2{x:310.0,y:30.0}, Align::Center );
          //draws the second answer 
          graphics::draw(_ctx,&dr_answer2,DrawParam{
            dest:Point2{x:410.0,y:410.0},
            ..Default::default()
          })?;
        }

        if self.a3_show == true {
          let mut dr_answer3 = Text::new(self.answer_3.as_str());
          dr_answer3.set_bounds(Point2{x:310.0,y:30.0}, Align::Center );
           //draws the third answer 
           graphics::draw(_ctx,&dr_answer3,DrawParam{
            dest:Point2{x:60.0,y:460.0},
            ..Default::default()
          })?;
        }

        if self.a4_show == true {
          let mut dr_answer4 = Text::new(self.answer_4.as_str());
          dr_answer4.set_bounds(Point2{x:310.0,y:30.0}, Align::Center );
            //draws the fourth answer 
            graphics::draw(_ctx,&dr_answer4,DrawParam{
              dest:Point2{x:410.0,y:460.0},
              ..Default::default()
            })?;

        }

         let mut dr_question = Text::new(self.question.as_str());//drawed question
         dr_question.set_bounds(Point2{x:680.0,y:70.0},Align::Center);
        
        
         //draws the question
          graphics::draw(_ctx,&dr_question,DrawParam{
              dest:Point2{x:60.0,y:310.0},
              ..Default::default()
          })?; 


        Ok(())
    }
  
}
