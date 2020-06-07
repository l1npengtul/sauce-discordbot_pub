use reqwest;
use serenity::{
    model::channel::Message,
    prelude::*,
};
use regex::Regex;
use tokio::runtime::Runtime;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, _ctx: Context, _new_message: Message) {
        println!("{:?}", _new_message.content);
        if _new_message.content.starts_with("n!") && _new_message.content.len() < 10{

            let digits_range = Regex::new("[\\d]{1,7}").unwrap().find(&_new_message.content).unwrap();
            let digits = &_new_message.content.to_owned()[digits_range.start()..digits_range.end()];
            let url_to_request = "https://ka.guya.moe/g/".to_owned()+&digits;






            let display_comic : Comic = Runtime::new().expect("Failed to start the Tokio Runtime").block_on(get_tags_and_send(&url_to_request));

            if display_comic.http_stat == reqwest::StatusCode::OK{
                let mut tag_as = String::from("");


                for t in &display_comic.tags{
                    tag_as.push_str(&t);
                    tag_as.push_str(", ");
                }
                //This is a hack. At the end we attach a carriage feed (newline, \n)
                //then we use regex to remove the last comma with it
                tag_as.push_str("\n");
                let end_rm_reg = Regex::new(", \n").unwrap();
                let tag_as_string = end_rm_reg.replace(&tag_as,"");
                println!("display_comic: {:?} {:?} {:?} {:?} {:?}", display_comic.http_stat, display_comic.author,display_comic.artist,display_comic.group,tag_as_string);

                if let Err(why) = _new_message.channel_id.send_message(&_ctx.http, |m | {
                    m.embed(|e | {
                        e.title(display_comic.title);
                        e.description(display_comic.n_link);
                        e.field("Author: ", display_comic.author,true);
                        e.field("Artist: ", display_comic.artist,true);
                        e.field("Group: ",display_comic.group,true);
                        e.field("Tags: ", tag_as_string[0..tag_as_string.len()].to_owned(),true);

                      e
                    });

                    m
                  }
                )
                {
                    println!("Error sending embed: {:?}", why);
                    if let Err(why) = _new_message.channel_id.say(&_ctx.http, "Error processing embed. It is possible that the query failed, or the bot is experiencing problems."){
                        println!("Error sending embed: {:?}", why);

                    }
                }
            }
            /*
            end_message(&ctx.http, |m | {
    m.embed(|e | {
      e.title("This is an embed!");
      e.description("This is a description of the embed!");

      return e;
    });

    return m;
  }
);
m.embed(|e |{
    e.title(display_comic.title);
    e.description(display_comic.n_link);
    e.field("Author: ", display_comic.author,true);
    e.field("Artist: ", display_comic.artist,true);
    e.field("Group: ",display_comic.group,true);
    e.field("Tags: ", tag_as_string,true);

    return e;
});
return m;
})
*/
            else{
                if let Err(why) = _new_message.channel_id.say(&_ctx.http,"Error: The Server Returned ".to_owned() + display_comic.http_stat.as_str() + ": *The server is either down or the comic could not be found!*"){
                    println!("Error sending message: {:?}", why);
                }
            }


        }
    }

}


struct Comic {
    http_stat : reqwest::StatusCode,
    author : String,
    artist : String,
    title : String,
    tags : Vec<String>,
    n_link : String,
    group : String,
}


async fn get_tags_and_send(url : &str) -> Comic {
    println!("here2");
    let mut base_ret_comic = Comic {
        http_stat: reqwest::StatusCode::SERVICE_UNAVAILABLE,
        author: String::from("chrome be stealin all my pink demon maids"),
        artist: String::from("fuck you borrow checker"),
        title: String::from("AAAAAAAAAAAAAAaaaaaaaaaaaaaaa"),
        tags: vec![String::from("i"),String::from("fucking"),String::from("hate"),String::from("priscilla")],
        n_link: String::from("who is rem"),
        group: String::from("i love emilia"),

    };
    let mut res_st = reqwest::StatusCode::NOT_FOUND;
    let mut site :String;
    site = String::from("");
    let _a = async {
        println!("a");
        let res = reqwest::get(url).await?;
        println!("{:?}", res);
        res_st = res.status();
        println!("{:?}", res_st);
        site = res.text().await?;
        println!("{:?}", site);
        Ok::<(), reqwest::Error>(())
    }.await;

    if res_st != reqwest::StatusCode::OK{
        println!("not 200");
        println!("SERVER RETURNED: {:?}", res_st.as_str());
        println!("URL : {:?}", url);
        base_ret_comic.http_stat = res_st;
        base_ret_comic
    }
    else{

        // I KNOW THAT PARSING HTML WITH REGEX IS A FUCKING SIN
        // BUT THIS DUMBASS WEBSITE DOESNT HAVE PROPER CLASSES FOR EACH TAG
        // GET OFF MY FUCKING BACK AAAAAAAAAAAAAA

        let auth_art_reg = Regex::new("<td class=\"text-sm\">[\\S\\s]{0,}</td>").unwrap();
        let title_tag_reg = Regex::new("<p>[\\s\\S]{0,}</p>").unwrap();
        let group_reg = Regex::new("<td scope=\"row\">[\\s\\S]{0,}</td>").unwrap();
        let original_link = Regex::new("<a href=\"https://nhentai.net/g/[\\d]{0,}/\">View Original</a>").unwrap();

        let mut is_auth : bool = true;
        let mut auth_art_counter = 0;
        let mut is_title : bool = true;
        let mut title_tag_counter = 0;

        let a = site;
        let b = a;
        println!("site: {:?}", b);
        let html_vec : Vec<&str> = b.lines().collect();

        let mut author : String = String::from("");
        let mut artist : String = String::from("");
        let mut title : String = String::from("");
        let mut tags : Vec<String> = Vec::new();
        let mut n_link : String = String::from("");
        let mut group : String = String::from("");




        for line in html_vec{
            println!("line: {:?}", line);
            if auth_art_reg.is_match(line){
                let start_reg = Regex::new("<td class=\"text-sm\">").unwrap();
                let end_reg = Regex::new("</td>").unwrap();

                if is_auth == true && auth_art_counter < 2{
                    let auth_rep_1 = start_reg.replace(line,"");
                    let auth_rep_2 = end_reg.replace(&auth_rep_1,"");
                    println!("auth {:?}", auth_rep_2);
                    author = auth_rep_2.to_owned().to_string();
                    is_auth = false;
                    auth_art_counter += 1;
                }
                else if is_auth == false && auth_art_counter < 2{
                    let auth_rep_1 = start_reg.replace(line,"");
                    let auth_rep_2 = end_reg.replace(&auth_rep_1,"");
                    println!("art {:?}", auth_rep_2);
                    artist = auth_rep_2.to_owned().to_string();
                    is_auth = false;
                    auth_art_counter += 1;
                }

            }
            else if title_tag_reg.is_match(line){
                let start_reg = Regex::new("<p>").unwrap();
                let end_reg = Regex::new("</p>").unwrap();

                if is_title == true && title_tag_counter < 2{
                    let auth_rep_1 = start_reg.replace(line,"");
                    let auth_rep_2 = end_reg.replace(&auth_rep_1,"");
                    println!("title {:?}", auth_rep_2);
                    title = auth_rep_2.to_owned().to_string();
                    is_title = false;
                    title_tag_counter += 1;
                }
                else if is_title == false && title_tag_counter < 2{
                    let auth_rep_1 = start_reg.replace(line,"");
                    let auth_rep_2 = end_reg.replace(&auth_rep_1,"");
                    let split_tags = auth_rep_2.split(" - ");
                    for s in split_tags{
                        tags.push(String::from(s));
                    }
                    println!("tags {:?}", tags);
                    is_title = false;
                    title_tag_counter += 1;
                }
            }
            else if group_reg.is_match(line){
                let start_reg = Regex::new("<td scope=\"row\">").unwrap();
                let end_reg = Regex::new("</td>").unwrap();

                let auth_rep_1 = start_reg.replace(line,"");
                let auth_rep_2 = end_reg.replace(&auth_rep_1,"");

                group = auth_rep_2.to_owned().to_string();
                println!("{:?}", group);

            }
            else if original_link.is_match(line){
                //<a href=\"https:\/\/nhentai.net\/g\/
                //\/\\">View Original<\/a>
                let start_reg = Regex::new("<a href=\"").unwrap();
                let end_reg = Regex::new("/\">View Original</a>").unwrap();

                let auth_rep_1 = start_reg.replace(line,"");
                let auth_rep_2 = end_reg.replace(&auth_rep_1,"");

                n_link = auth_rep_2.to_owned().to_string();
                println!("{:?}", n_link);
            }
        }
        let var_name = res_st;
        let res_status = var_name;

        println!("ret_comic: {:?} {:?} {:?} {:?} {:?} {:?} {:?}", res_status,author,artist,title,tags,n_link,group);

        let ret_comic = Comic{
            http_stat : res_status,
            author,
            artist,
            title,
            tags,
            n_link,
            group,

        };
        ret_comic
    }
}



/*
fn get_tags(url: &String) -> Comic{
    try!()
}



async fn get_site()-> Result<(), Box<dyn Error>>{
    let content = reqwest::get("http://httpbin.org/range/26")
    .await?
    .text()
    .await?;
    println!("text: {:?}", content);
    Ok(())

}
*/

fn main(){
    let token = "<token>";

    let mut discord_bot = Client::new(&token, Handler).expect("Error creating client");

    if let Err(why) = discord_bot.start(){
        println!("Client Error: {:?}", why);
    }

}
