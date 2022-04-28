#[cfg(test)]
use mockall::automock;

use clap::{Args, Parser, Subcommand, ArgEnum};

#[cfg_attr(test, automock)]
pub trait Writer {
    fn write(&self, file_name: &str, content: &str) -> Option<Box<dyn std::error::Error>>;
}

#[cfg_attr(test, automock)]
pub trait Time {
    fn cur_date_time(&self, format: &str) -> String;
}

#[derive(Debug, Parser)]
#[clap(name = "gk-yaml - goalkeeper yaml generator")]
#[clap(version = "0.1.0")]
#[clap(about = "A CLI application that helps to work with goalkeeper generating yaml files automatically.", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,

    #[clap(short, long, required = true)]
    /// The slack channel to send tasks notifications.
    pub channel: String,

    #[clap(short, long, required = true)]
    /// Squad related to the task. It is used to create file path.
    pub squad: String,

    #[clap(short, long, required = true)]
    /// Directory that [goalkeeper](https://github.com/pismo/goalkeeper) is located.
    pub root_directory: String,

    #[clap(required = true, multiple_values = true, short, long, arg_enum)]
    /// Environment related to the task. It accepts a list: ext,itau,multitenant.
    pub envs: Vec<Env>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    SQS(SQS),
}

#[derive(Debug, Args)]
pub struct SQS {
    #[clap(subcommand)]
    pub command: Option<SQSCommand>,
}

#[derive(Debug, ArgEnum, Clone)]
pub enum Env {
    Itau,
    Multitenant,
    India,
    Ext,
}

impl Env {
    pub fn path(&self) -> String {
        match self {
            Env::Ext => String::from("ext"),
            Env::Itau => String::from("itau"),
            Env::Multitenant => String::from("prod"),
            Env::India => String::from("ind-prod"),
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum SQSCommand {
    Download {
        #[clap(required = true, short, long)]
        queue: String
    },
    Reprocess {
        #[clap(required = true, short, long)]
        src_queue: String,

        #[clap(required = true, short, long)]
        dst_queue: String,
    },
}

impl Commands {
    pub fn build_yaml(&self, cur_date_time: &str, channel: &str) -> (String, String) {
        match self {
            Commands::SQS(sqs_command) => {
                match sqs_command.command.as_ref().unwrap() {
                    SQSCommand::Download { queue } => ("download_sqs".to_string(), build_download_yaml(cur_date_time, channel, &queue)),
                    SQSCommand::Reprocess { src_queue, dst_queue } => ("reprocess_sqs".to_string(), build_reprocess_yaml(cur_date_time, channel, &src_queue, &dst_queue)),
                }
            }
        }
    }
}

fn build_download_yaml(cur_date_time: &str, channel: &str, queue: &str) -> String {
    format!(
        r#"apiVersion: argoproj.io/v1alpha1
kind: Workflow
metadata:
  name: {}-download
  namespace: goalkeeper
spec:
  entrypoint: wf-entrypoint
  serviceAccountName: goalkeeper
  activeDeadlineSeconds: 600
  templates:
    - name: wf-entrypoint
      dag:
        tasks:
          - name: sqs
            templateRef:
              name: aws
              template: queue-to-file-cli
            arguments:
              parameters: [
                {{ name: "source", value: '{}'}},
                {{ name: "region", value: 'sa-east-1'}}
              ]
          - name: slack-send-file
            dependencies: [sqs]
            templateRef:
              name: slack
              template: slack-send-file
            arguments:
              artifacts:
                - name: file
                  from: "{{{{tasks.sqs.outputs.artifacts.output}}}}"
              parameters: [
                {{
                  name: "message",
                  value: "Task {{{{workflow.name}}}}"
                }},
                {{
                  name: "filename",
                  value: "{}-download.json.gz"
                }},
                {{
                  name: "channel",
                  value: "{}"
                }}
              ]
    "#, cur_date_time, queue, cur_date_time, channel)
}

fn build_reprocess_yaml(cur_date_time: &str, channel: &str, src_queue: &str, dst_queue: &str) -> String {
    format!(
        r#"apiVersion: argoproj.io/v1alpha1
kind: Workflow
metadata:
  name: {}-reprocess
  namespace: goalkeeper
spec:
  entrypoint: wf-entrypoint
  serviceAccountName: goalkeeper
  templates:
    - name: wf-entrypoint
      dag:
        tasks:
          - name: sqs
            templateRef:
              name: aws
              template: dlq-to-queue
            arguments:
              parameters:
                [
                  {{
                    name: "source",
                    value: "{}",
                  }},
                  {{
                    name: "destination",
                    value: "{}",
                  }},
                  {{ name: "region", value: "sa-east-1" }},
                  {{ name: "batch", value: 1 }},
                ]
          - name: slack-send-file
            dependencies: [sqs]
            templateRef:
              name: slack
              template: slack-send-file
            arguments:
              artifacts:
                - name: file
                  from: "{{{{tasks.sqs.outputs.artifacts.output}}}}"
              parameters:
                [
                  {{ name: "message", value: "Task {{{{workflow.name}}}}" }},
                  {{ name: "filename", value: "logs.txt" }},
                  {{ name: "channel", value: "{}" }},
                ]
    "#, cur_date_time, src_queue, dst_queue, channel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_description() {
        assert_eq!(Env::Ext.path(), "ext");
        assert_eq!(Env::India.path(), "ind-prod");
        assert_eq!(Env::Itau.path(), "itau");
        assert_eq!(Env::Multitenant.path(), "prod");
    }

    #[test]
    fn build_yaml_for_download() {
        let exp = (String::from("download_sqs"),
                   String::from(format!(
                       r#"apiVersion: argoproj.io/v1alpha1
kind: Workflow
metadata:
  name: 270422-082632-download
  namespace: goalkeeper
spec:
  entrypoint: wf-entrypoint
  serviceAccountName: goalkeeper
  activeDeadlineSeconds: 600
  templates:
    - name: wf-entrypoint
      dag:
        tasks:
          - name: sqs
            templateRef:
              name: aws
              template: queue-to-file-cli
            arguments:
              parameters: [
                {{ name: "source", value: 'queue'}},
                {{ name: "region", value: 'sa-east-1'}}
              ]
          - name: slack-send-file
            dependencies: [sqs]
            templateRef:
              name: slack
              template: slack-send-file
            arguments:
              artifacts:
                - name: file
                  from: "{{{{tasks.sqs.outputs.artifacts.output}}}}"
              parameters: [
                {{
                  name: "message",
                  value: "Task {{{{workflow.name}}}}"
                }},
                {{
                  name: "filename",
                  value: "270422-082632-download.json.gz"
                }},
                {{
                  name: "channel",
                  value: "slackChannel"
                }}
              ]
    "#)));
        let cur_date_time = String::from("270422-082632");
        let queue = String::from("queue");
        let channel = String::from("slackChannel");
        let reprocess_command = Some(SQSCommand::Download { queue });
        let sqs_command = SQS { command: reprocess_command };

        let cli = Cli {
            command: crate::types::Commands::SQS(sqs_command),
            channel: String::from(""),
            squad: String::from(""),
            root_directory: String::from(""),
            envs: Vec::new(),
        };
        let (act_relative_name, act_content) = cli.command.build_yaml(&cur_date_time, &channel);
        let (exp_relative_name, exp_content) = exp;

        assert_eq!(act_relative_name, exp_relative_name);
        assert_eq!(act_content, exp_content);
    }

    #[test]
    fn build_yaml_for_reprocess() {
        let exp = (String::from("reprocess_sqs"),
                   String::from(format!(r#"apiVersion: argoproj.io/v1alpha1
kind: Workflow
metadata:
  name: 270422-082632-reprocess
  namespace: goalkeeper
spec:
  entrypoint: wf-entrypoint
  serviceAccountName: goalkeeper
  templates:
    - name: wf-entrypoint
      dag:
        tasks:
          - name: sqs
            templateRef:
              name: aws
              template: dlq-to-queue
            arguments:
              parameters:
                [
                  {{
                    name: "source",
                    value: "src",
                  }},
                  {{
                    name: "destination",
                    value: "dst",
                  }},
                  {{ name: "region", value: "sa-east-1" }},
                  {{ name: "batch", value: 1 }},
                ]
          - name: slack-send-file
            dependencies: [sqs]
            templateRef:
              name: slack
              template: slack-send-file
            arguments:
              artifacts:
                - name: file
                  from: "{{{{tasks.sqs.outputs.artifacts.output}}}}"
              parameters:
                [
                  {{ name: "message", value: "Task {{{{workflow.name}}}}" }},
                  {{ name: "filename", value: "logs.txt" }},
                  {{ name: "channel", value: "slackChannel" }},
                ]
    "#)));
        let cur_date_time = String::from("270422-082632");
        let src_queue = String::from("src");
        let dst_queue = String::from("dst");
        let channel = "slackChannel".to_string();
        let reprocess_command = Some(SQSCommand::Reprocess { src_queue, dst_queue });
        let sqs_command = SQS { command: reprocess_command };

        let cli = Cli {
            command: crate::types::Commands::SQS(sqs_command),
            channel: String::from(""),
            squad: String::from(""),
            root_directory: String::from(""),
            envs: Vec::new(),
        };
        let (act_relative_name, act_content) = cli.command.build_yaml(&cur_date_time, &channel);
        let (exp_relative_name, exp_content) = exp;

        assert_eq!(act_relative_name, exp_relative_name);
        assert_eq!(act_content, exp_content);
    }
}

