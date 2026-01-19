use std::env;
use dotenvy::dotenv;
use serenity::async_trait;
use tracing::{info, error, Level};

// We need the rand crate for the /roll command, so we'll add it here.
use rand::Rng;

// Using serenity::all::* as the primary import.
use serenity::all::*; 

// We need CreateCommand and CreateInteractionResponseMessage for building.
use serenity::builder::{CreateInteractionResponseMessage, CreateCommand, CreateCommandOption}; 

// Define the handler for events sent by Discord.
struct Handler;

// Helper function to extract a command option's value by name
fn get_option_value<'a>(
    options: &'a [CommandDataOption],
    name: &str,
) -> Option<&'a CommandDataOptionValue> {
    options.iter()
        .find(|opt| opt.name == name)
        .map(|opt| &opt.value)
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction { 
            info!("Received command interaction: {:?}", command.data.name);

            let content = match command.data.name.as_str() {
                "ping" => "Pong! I'm alive and using slash commands.",

                "notify" => {
                    // Take a list of people
                    // Extract the 'target_user' option value
                    let target_user = get_option_value(&command.data.options, "target_user")
                        .and_then(|val| if let CommandDataOptionValue::User(u) = val { Some(u) } else { None });

                    // Extract the 'message' option value
                    let message = get_option_value(&command.data.options, "message")
                        .and_then(|val| if let CommandDataOptionValue::String(s) = val { Some(s) } else { None });

                    &match (target_user, message) {
                        (Some(user), Some(msg)) => {
                            // 1. Try to create a Direct Message channel with the target user
                            match user.create_dm_channel(&ctx.http).await {
                                Ok(dm_channel) => {
                                    // 2. Try to send the message to the DM channel
                                    let dm_content = format!(
                                        "🔔 **Jira Task Reminder** from {}: {}", 
                                        command.user.name, 
                                        msg
                                    );
                                    
                                    if let Err(why) = dm_channel.say(&ctx.http, dm_content).await {
                                        error!("Error sending DM: {:?}", why);
                                        format!("I failed to send a DM to {}. They may have DMs disabled. Error: {}", user.mention(), why)
                                    } else {
                                        format!("✅ Successfully sent the notification to **{}** via DM.", user.mention())
                                    }
                                }
                                Err(why) => {
                                    error!("Error creating DM channel: {:?}", why);
                                    format!("I couldn't open a DM channel with {}. Error: {}", user.mention(), why)
                                }
                            }
                        },
                        _ => "Missing user or message for the notification.".to_string(),
                    }
                },
                
                "greet" => {
                    // Extract the 'user' option value
                    &if let Some(CommandDataOptionValue::User(user)) = 
                        get_option_value(&command.data.options, "user") 
                    {
                        // The User object contains the user's ID, which we format as a mention
                        format!("Hello, {}! I've been configured to greet you.", user.mention())
                    } else {
                        "Couldn't find the user to greet. Are you sure you selected one?".to_string()
                    }
                },
                
                "roll" => {
                    // Extract the 'max' option value
                    &if let Some(CommandDataOptionValue::Integer(max_int)) = 
                        get_option_value(&command.data.options, "max") 
                    {
                        let max = *max_int;
                        if max <= 0 {
                            "You must roll a number greater than 0.".to_string()
                        } else {
                            // Generate a random number between 1 and max (inclusive)
                            let roll = rand::thread_rng().gen_range(1..=(max as u64));
                            format!("🎲 You rolled a **{}** (1-{})", roll, max)
                        }
                    } else {
                        "Invalid input for the max number. Please enter a valid integer.".to_string()
                    }
                },
                
                _ => "Unknown command.",
            };

            // CORRECT RESPONSE STRUCTURE for 0.12.4:
            // 1. Create the inner message structure (CreateInteractionResponseMessage).
            let response_message = CreateInteractionResponseMessage::new()
                .content(content);
                
            // 2. Construct the CreateInteractionResponse enum by wrapping the message.
            let interaction_response = CreateInteractionResponse::Message(response_message);

            // Respond to the command interaction by passing the complete enum object.
            if let Err(why) = command
                .create_response(&ctx.http, interaction_response)
                .await
            {
                error!("Cannot respond to slash command: {:?}", why);
            }
        }
    }

    // 2. Called when the bot successfully connects and is ready.
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected and ready!", ready.user.name);

        // Define ALL commands to be registered globally
        let commands = vec![
            // 1. /ping command
            CreateCommand::new("ping")
                .description("A simple test command to see if the bot is responsive."),

            // 2. /notify <target_user> <message> command
            CreateCommand::new("notify")
                .description("Sends a personal task reminder to a user via Direct Message (DM).")
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::User, 
                        "target_user", 
                        "The user to send the reminder to."
                    )
                    .required(true),
                )
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String, 
                        "message", 
                        "The task/reminder message."
                    )
                    .required(true),
                ),

            // 3. /greet <user> command
            CreateCommand::new("greet")
                .description("Greets a specific user.")
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::User, // The option type is a User mention
                        "user", 
                        "The user you want to greet."
                    )
                    .required(true),
                ),
                
            // 4. /roll <max> command
            CreateCommand::new("roll")
                .description("Rolls a random number between 1 and the specified maximum.")
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::Integer, // The option type is an Integer
                        "max", 
                        "The maximum number to roll up to (e.g., 20 for a D20)."
                    )
                    .required(true),
                ),
        ];

        // Set all global commands at once
        let result = Command::set_global_commands(&ctx.http, commands).await;

        match result {
            Ok(cmds) => info!("Successfully registered {} global commands.", cmds.len()),
            Err(why) => error!("Failed to register global commands: {:?}", why),
        }
    }
    
    // 3. The message handler remains as an empty stub.
    async fn message(&self, _ctx: Context, _msg: Message) {}
}

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to read .env file");

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected DISCORD_TOKEN in the environment");

    // Intents necessary for command handling and basic messaging.
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILDS 
        | GatewayIntents::GUILD_INTEGRATIONS // for slash
        | GatewayIntents::GUILD_MEMBERS; // for notify

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}