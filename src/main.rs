/* An exemple of how to use Choose_Your_Language */

use choose_your_language::ChooseYourLanguage;
use choose_your_language::Message;

fn main() {
    let mut message: Message = Message::new(false, String::from("je mange du riz"), '.');
    let dict_value: Vec<String> = vec!["l", "a", "v", "i", "/", "s", "t", "b", "ù", "m", "8", "£", "$", "+", "µ", "q", "#", "@", "^", "ç", "&", "`", ">", "*", "à", "~"]
    .into_iter().map(|c| c.to_string()).collect();
    message.make_dict(dict_value);

    let crypt_message: String = message.crypt();

    println!("The encrypted message is: {}", crypt_message);


    let mut dmessage: Message = Message::new(true, String::from("m./. .$.l.+.t./. .i.&. .@.ù.~."), '.');
    let ddict_value: Vec<String> = vec!["l", "a", "v", "i", "/", "s", "t", "b", "ù", "m", "8", "£", "$", "+", "µ", "q", "#", "@", "^", "ç", "&", "`", ">", "*", "à", "~"]
    .into_iter().map(|c| c.to_string()).collect();
    dmessage.make_dict(ddict_value);

    let dcrypt_message: String = dmessage.decrypt();

    println!("The clear message corresponding is: {}", dcrypt_message);
}
