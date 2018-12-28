use tbot::{prelude::*, types::ParseMode::Markdown, Bot};

fn main() {
    let mut bot = Bot::from_env("BOT_TOKEN");

    bot.on_message(|context| {
        let message = match meval::eval_str(&context.message) {
            Ok(result) => format!("= `{}`", result),
            Err(_) => "Whops, I couldn't evaluate your expression :(".into(),
        };

        let reply = context
            .send_message_in_reply(&message)
            .parse_mode(Markdown)
            .into_future()
            .map_err(|error| eprintln!("Whops, got an error: {:#?}", error));

        tbot::spawn(reply);
    });

    bot.polling().start();
}