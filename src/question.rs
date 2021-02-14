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
            
        }
    }

    //draw and update should be added
  
}

/*
impl Clone for Question{
    fn clone(&self) -> Self{
        Self{
            question:self.question.clone(),
            answer_1:self.answer_1.clone(),
            answer_2:self.answer_2.clone(),
            answer_3:self.answer_3.clone(),
            answer_4:self.answer_4.clone(),
            correct_answer:self.correct_answer.clone()
        }
    }
}
*/