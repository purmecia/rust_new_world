use rust_bert::pipelines::question_answering::{
    QaInput, QuestionAnsweringConfig, QuestionAnsweringModel,
};
use std::io;
use tokio::runtime::Runtime;

fn main() {
    let config = QuestionAnsweringConfig::default();
    let qa_model = QuestionAnsweringModel::new(config).unwrap();

    let mut rt = Runtime::new().unwrap();

    println!("Question Answering model is ready!");

    loop {
        println!("Enter a context paragraph:");
        let mut context = String::new();
        io::stdin()
            .read_line(&mut context)
            .expect("Failed to read context");
    
        if context.trim().is_empty() {
            break;
        }
    
        println!("Enter a question based on the context:");
        let mut question = String::new();
        io::stdin()
            .read_line(&mut question)
            .expect("Failed to read question");
    
        let qa_input = QaInput {
            question: question.trim().to_string(),
            context: context.trim().to_string(),
        };
    
        let answers = rt.block_on(async { qa_model.predict(&[qa_input], 1, 32) });

        if let Some(answers) = answers.into_iter().next() {
            if let Some(answer) = answers.get(0) {
                println!("Answer: {}\n", answer.answer.clone());
            } else {
                println!("No answer found.\n");
            }
        } else {
            println!("No answer found.\n");
        }
    }
}    