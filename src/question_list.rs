use crate::question::Question;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;


pub struct QuestionList{
   pub question_list:Vec<Question>,
   pos:usize 
}

impl QuestionList{
    pub fn new() ->Self{
        let f = File::open("C:/Users/Симеон/Desktop/Rust/wwtm_project/resources/question_data.txt").unwrap();
        let reader = BufReader::new(f);
        let mut _question_list = Vec::<Question>::new();

        for l in reader.lines(){
            let question = Question::new(l.unwrap());
            _question_list.push(question);
        }

        Self{ question_list: _question_list, pos: 0 }
    }
}




impl Iterator for QuestionList{
    type Item = Question;
    fn next(&mut self) -> Option<Self::Item>{
        
        let mut iterator = self.question_list.iter();
        let  current = iterator.nth(self.pos); 

        match current{
            Some(_) => {
                self.pos+=1;
                Some(current.unwrap().clone())
            },
            None => None
        }      
    }
    
}