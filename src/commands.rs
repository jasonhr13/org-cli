use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::time::Duration;
use std::process::Command;
use inquire::Select;
use inquire::ui::{Color, RenderConfig, Styled};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2 as awsec2;
use aws_sdk_secretsmanager as secrets;
use std::env;
use indicatif::{ProgressBar, ProgressStyle};

mod types;

pub async fn ssh_all() {
    let client = ec2_client().await;
    let secclient = sec_client().await;
    let pb = start_progress_with_message("Fetching instances from AWS".to_string());
    let instance_arr = desc_instances(&client.unwrap()).await;
    let options = instance_arr.unwrap();

    pb.finish_with_message("Done");

    let ans = selection(options).prompt();

    match ans {
        Ok(choice) => {
            trigger_connection(secclient.as_ref().unwrap(), choice).await;
        },
        Err(_) => println!("There was an error, please try again"),
    }
}

pub async fn ssh(server: String, production: bool, a: bool, b: bool) {
    let client = ec2_client().await;
    let secclient = sec_client().await;
    let mut found: bool = false;
    let pb = start_progress_with_message("Searching for instance in AWS".to_string());
    let mut instance_arr = desc_instances(&client.unwrap()).await.unwrap_or_default();

    pb.finish();

    if production {
        instance_arr.retain(|inst| inst.name == server.to_string());

        if a || b {
            for inst in instance_arr {
                let zone: String;
                if a {
                    zone = "us-west-2a".to_string();
                } else {
                    zone = "us-west-2b".to_string();
                }
                if inst.az == zone {
                    found = true;
                    trigger_connection(secclient.as_ref().unwrap(), inst).await;
                }
            }
        } else {
            let ans = selection(instance_arr).prompt();

            match ans {
                Ok(choice) => {
                    found = true;
                    trigger_connection(secclient.as_ref().unwrap(), choice).await;
                },
                Err(_) => {
                    found = true;
                    println!("There was an error, please try again")
                },
            }
        }
    }   else {
        for inst in instance_arr{
            if inst.name == capitalize(&server) {
                found = true;
                trigger_connection(secclient.as_ref().unwrap(), inst).await;
            }
        }
    }

    if !found {
        println!("That server does not seem to be online.");
    }
}

fn start_progress_with_message(msg: String) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    ProgressStyle::with_template("{spinner.blue} {msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    pb.set_message(msg);
    return pb;
}

fn selection(instances: Vec<types::Instance>) -> Select<'static, types::Instance> {
    let default: RenderConfig = RenderConfig::default();
    let highlighted_option_prefix = Styled::new(">>>").with_fg(Color::LightGreen);
    let mine = default.with_highlighted_option_prefix(highlighted_option_prefix);
    return Select::new("Select a server", instances).with_page_size(5).with_render_config(mine);
}

async fn trigger_connection(secclient: &aws_sdk_secretsmanager::Client, choice: types::Instance) {
    update_key_file(&secclient, choice.keyname.to_string()).await;
    let file_location = format!("{}/.ssh/{}.pem", env::var_os("HOME").unwrap().into_string().unwrap_or_default().trim(), choice.keyname.to_string());
    let host = format!("ec2-user@{}", choice.ip);

    let mut child = Command::new("ssh")
                            .arg("-i")
                            .arg(file_location)
                            .arg(host)
                            .spawn()
                            .unwrap();
    let _asd = child.wait().unwrap();
}

async fn ec2_client() -> Result<aws_sdk_ec2::Client, aws_sdk_ec2::Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    Ok(awsec2::Client::new(&config))
}

async fn sec_client() -> Result<aws_sdk_secretsmanager::Client, aws_sdk_secretsmanager::Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    Ok(secrets::Client::new(&config))
}

async fn update_key_file(client: &secrets::Client, name: String) -> std::io::Result<()>{
    let resp = client.get_secret_value().secret_id(format!("ssh/{}", name)).send().await;

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(format!("{}/.ssh/{}.pem", env::var_os("HOME").unwrap().into_string().unwrap_or_default(), name))?;

    file.write_all(resp.unwrap()
                       .secret_string()
                       .unwrap()
                       .to_string()
                       .as_bytes()).unwrap();

    file.sync_all()?;
    Ok(())
}

async fn desc_instances(client: &awsec2::Client) -> Result<Vec<types::Instance>, awsec2::Error> {
    let mut instances = Vec::new();
    let resp = client
        .describe_instances()
        .send()
        .await?;

    for reservation in resp.reservations().unwrap_or_default() {
        for instance in reservation.instances().unwrap_or_default() {
            if instance.private_ip_address().unwrap_or_default().to_string() == "" {
                continue;
            }

            if instance.state().unwrap().code().unwrap() != 16 {
                continue;
            }

            let tag = instance.tags().unwrap_or_default().iter().find(|&x| x.key().unwrap_or_default() == "Name");
            let inst = types::Instance {
                name: tag.unwrap().value().unwrap_or_default().to_string(),
                id: instance.instance_id().unwrap().to_string(),
                ip: instance.private_ip_address().unwrap_or_default().to_string(),
                keyname: instance.key_name().unwrap().to_string().replace(" ", ""),
                az: instance.placement().unwrap().availability_zone().unwrap_or_default().to_string()
            };
            instances.push(inst);
        }
    }

    Ok(instances)
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
