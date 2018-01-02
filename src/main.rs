extern crate clap;

use clap::{Arg, App, ArgMatches, SubCommand};
use std::env;
use std::process::Command;
use std::error::Error;
use std::io::{Write, BufWriter};
use std::fs::OpenOptions;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;

// TODO use environment variables and flags to customize
static JRNL_HOME: str = "~/.jrnl"
static JRNL_LOG: str = "~/.jrnl/events.log"

#[derive(Debug)]
struct Note {
    title: String,
    project: String,
    // TODO tags, datetime

    updated_at: u64,
}

impl Note {
    fn tmp_file(&self) -> String {
        // determinist and unique filename
        format!("/{}/notes/{}.{}.md", JRNL_HOME, self.project, str::replace(&self.title, " ", "-"))
    }

    fn edit(&self, user_editor: String) {
        println!("using editor: '{}'", editor());
        // TODO generate random key from title
        Command::new(user_editor)
                .arg(self.tmp_file())
                .status()
                .expect("failed to open editor");
    }
}

fn new_note(title: String) -> Note {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    Note {
        title,
        // TODO read project from some kind of persistent info
        project: String::from("training"),
        updated_at: since_the_epoch.as_secs(),
    }
}

fn editor() -> String {
    // TODO default editor
    env::var("EDITOR").unwrap()
}

fn edit_note(matches: &ArgMatches) {
    let title = matches.value_of("TITLE").unwrap();
    let tags = matches.value_of("tags").unwrap();

    let note = new_note(String::from(title));
    // TODO editor = cli arg > environment > default value
    // NOTE could return the file where it was written
    note.edit(editor());

    // https://stackoverflow.com/questions/26643688/how-to-split-a-string-in-rust
    let event_log = format!("event={} tags={} filename={} updated_at={}\n", "edit", tags, note.tmp_file(), note.updated_at);

    let fd = match OpenOptions::new().create(true).append(true).open(JRNL_LOG) {
        Ok(file) => file,
        Err(why) => panic!("at the Disco: {}", why.description()),
    };

    //let fd = File::open("/tmp/journal.log").expect("unable to create journal file");
    let mut writer = BufWriter::new(&fd);
    writer.write_all(event_log.as_bytes()).expect("unable to log event");
}

fn remove_jounral() {
    // FIXME share journal name...
    fs::remove_file(JRNL_LOG).unwrap_or_else(|why| {
        println!("failed to remove file: {:?}", why.kind());
    });
    // NOTE should I also remove all the snippets ? cli flag ?
}


fn main() {
    // TODO use subcommmands
    // TODO use `multiple(true)` for tags
    let matches = App::new("journal")
                       .version("0.1.0")
                       .about("structure micro knowledge")
                       .author("Xavier B.")
                       .subcommand(SubCommand::with_name("clear")
                                   .about("reset journal content"))
                       .subcommand(SubCommand::with_name("edit")
                                   .about("edit a new or existing note")
                                   .arg(Arg::with_name("tags")
                                           .short("t")
                                           .long("tag")
                                           .value_name("TAG")
                                           .takes_value(true)
                                           .help("bind tags with note"))
                                   .arg(Arg::with_name("TITLE")
                                           .help("note title")
                                           .default_value("thoughts")
                                           .index(1)))
                       .get_matches();

    match matches.subcommand() {
        ("edit", Some(m))  => edit_note(m),
        ("clear", Some(_)) => remove_jounral(),
        _                  => {},  // Either no subcommand or one not tested for...
    }
}
