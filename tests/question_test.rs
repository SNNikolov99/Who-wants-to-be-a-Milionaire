use wwtm_project::question::Question;
use wwtm_project::question_list::QuestionList;


#[test]
fn test_question() {
    let _str = String::from(
    "Кой град е столица на България?/
    София/
    Пловдив/
    Варна/
    Бургас/
    a"
    );

    let question_test = Question::new(_str);
    assert_eq!("Кой град е столица на България?",question_test.question);
    assert_eq!("София",question_test.answer_1);
    assert_eq!("Пловдив",question_test.answer_2);
    assert_eq!("Варна",question_test.answer_3);
    assert_eq!("Бургас",question_test.answer_4);
    assert_eq!('a',question_test.correct_answer);


}

#[test]
fn test_question_list() {

    let mut question_list = QuestionList::new();
    assert_eq!("Коя година сме сега?",question_list.next().unwrap().question);
    assert_eq!("Колко месеца има в годината?",question_list.next().unwrap().question);


}