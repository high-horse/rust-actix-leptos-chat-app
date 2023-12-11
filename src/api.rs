use std::fmt::format;

use leptos::{*, html::Data};

use crate::model::conversation::{Conversation, Message};

#[server(Converse "/api")]
pub async fn converse(prompt: Conversation) ->Result<String, ServerFnError> {
    use llm::models::Llama;
    use leptos_action::extract;
    use actix_web::web::Data;
    use actix_web::dev::ConnectionInfo;

    let model = extract( |data: Data<Llama>, _connection: ConnectionInfo| async {
        data.into_inner() 
    })
    .await.unwrap();

    use llm::KnownModel;
    let character_name = "### Assistant";
    let user_name = "### Human";
    let person = "A chat between human and assistant";
    let mut history = format!("
        {character_name}:Hello How may I help you today?\n\
        {user_name}:What is the capital of France?\n\
        {character_name}:Paris is the caital of France.\n\
    ");

    for message in prompt.messages.into_iter() {
        let msg = message.text;
        let curr_line = if message.user {
            format!("{character_name}:{msg}\n");
        } else {
            format!("{user_name}:{msg}\n")
        };

        history.push_str(&curr_line);
    }

    let mut res = String::new();
    let mut rng = rand::thread_rnd();
    let mut buf = String::new();

    let mut session = model.start_session(Default::default());
    
    session.infer(
        model.as_ref(),
        &mut rng,
        &llm::InferenceRequest {
            prompt: format!("{persona}\n{history}\n{character_name}:")
                .as_str()
                .into(),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_token: false,
            maximum_token_count: None,
        },
        &mut Default::default(),
        inference_callback(String::fron(user_name), &mut buf, &mut res),
    )
    .unwrap_or_else(|e| panic!("{e}"));

    Ok(String::from(""))
}