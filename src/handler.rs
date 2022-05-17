use crate::types::{Cli, Writer, Time};

const DATE_FORMAT_STR: &'static str = "%d%m%Y-%H%M%S";

pub struct Handler {
    writer: Box<dyn Writer>,
    time: Box<dyn Time>,
}

impl Handler {
    pub fn new(writer: Box<dyn Writer>, time: Box<dyn Time>) -> Handler {
        Handler {
            writer,
            time,
        }
    }

    pub fn handle(&self, args: Cli) -> Result<(), Box<dyn std::error::Error>> {
        let cur_date_time = self.time.cur_date_time(DATE_FORMAT_STR);
        let command = args.command;
        let (relative_name, content) = command.build_yaml(&cur_date_time);
        let root_directory = args.root_directory;
        let squad = args.squad;

        for env in args.envs {
            let path = format!("{}/tasks/{}/{}/{}-{}.yaml", root_directory, env.path(), squad, cur_date_time, relative_name);
            self.writer.write(&path, &content);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{MockTime, MockWriter, Cli, Env, SQS, SQSCommand};
    use mockall::predicate::{always, function};
    use regex::Regex;

    #[test]
    fn must_call_writer_properly() -> Result<(), Box<dyn std::error::Error>> {
        const EXP_FORMAT: &'static str = "%d%m%Y-%H%M%S";
        let envs = vec![Env::India, Env::Ext];
        let channel = String::from("channel");
        let download_command = Some(SQSCommand::Download { queue: String::from(""), channel });
        let sqs_command = SQS { command: download_command };
        let args = Cli {
            squad: String::from("my-squad"),
            envs,
            root_directory: String::from("root_dir"),
            command: crate::types::Commands::SQS(sqs_command),
        };
        let mut mock_writer = MockWriter::new();
        mock_writer.expect_write()
            .with(function(|x: &str| {
                let re = Regex::new(r"root_dir/tasks/(ind-prod|ext)/my-squad/010101-download_sqs.yaml").unwrap();
                re.is_match(x)
            }), always())
            .times(2)
            .returning(|_, _| None);
        let mut mock_time = MockTime::new();
        mock_time.expect_cur_date_time()
            .with(function(|x: &str| EXP_FORMAT == x))
            .once()
            .returning(|_| String::from("010101"));


        let handler = Handler::new(Box::new(mock_writer), Box::new(mock_time));

        handler.handle(args)
    }
}

