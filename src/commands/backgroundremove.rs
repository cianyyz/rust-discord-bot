use serenity::model::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    http::AttachmentType,
    model::channel::Message,
    prelude::*,
};

//use std::io::prelude::*;
//use std::fs::File;

use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::io::Write;
use std::path::Path;
use std::fs;
use std::process::Command;
#[command]
#[description = "Removes image background"]
#[owners_only]
async fn bgrm(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    
    for attachment in &message.attachments {
            println!("{}", attachment.filename);
            let content = attachment.download().await?;
            let fileid = &format!("./tmp/{}",  attachment.filename);
            let resultid = &format!("./tmp/fixed/{}",  attachment.filename);
            let file = File::create(fileid).await;
             println!("{}", fileid);
            fs::write(fileid, content).expect("Unable to write file");
            // let _ =message.channel_id.say(&context, &format!("Saved {:?}", attachment.filename)).await;
            let output = Command::new("python")
                .arg("./src/functions/background_remove.py")
                .arg(&attachment.filename)
                .output().unwrap_or_else(|e| {
                    panic!("failed to execute process: {}", e)
            });

            if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

            print!("Background Removal succeeded and stdout was:\n{}", s);

            message.channel_id
            .send_message(&context.http, |m| {
                m.content("Background Remove Attempt");
                m.add_file(AttachmentType::Path(Path::new(resultid)));
                m
            })
            .await;
        } else {
            let s = String::from_utf8_lossy(&output.stderr);

            print!("Background Removal failed and stderr was:\n{}", s);
        }

            
    }
    Ok(())
}
