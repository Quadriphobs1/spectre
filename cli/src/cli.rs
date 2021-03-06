use clap::{
  crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, Shell,
  SubCommand,
};
use connection::Provider;

pub fn build_cli() -> App<'static, 'static> {
  App::new(crate_name!())
    .version(&*crate_version!())
    .about(crate_description!())
    .author(crate_authors!())
    .setting(AppSettings::VersionlessSubcommands)
    .after_help(
      "You can also run `spectre SUBCOMMAND -h` to get more information about that subcommand.",
    )
    .subcommand(migration_subcommand())
    .subcommand(seed_subcommand())
    .subcommand(init_subcommand())
    .subcommand(completions_subcommand())
    .setting(AppSettings::SubcommandRequiredElseHelp)
}

fn connection_arg<'a, 'b>() -> Arg<'a, 'b> {
  Arg::with_name("connection")
      .long("connection")
      .short("c")
      .alias("connection")
      .help(
          "Specify the connection to run the command again. By default this uses `default` connection defined in your config",
      )
      .takes_value(true)
      .global(true)
}

pub fn init_subcommand<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("init")
    .about(
      "Generates initial Spectre configuration file. \
        If name specified then creates files inside \
        directory called as name. If its not specified \
        then creates files inside current directory.",
    )
    .setting(AppSettings::VersionlessSubcommands)
    .args(&[
      Arg::with_name("docker")
        .alias("docker")
        .long("docker")
        .help("Flag if docker-compose must be generated as well.")
        .required(false)
        .requires("containers"),
      Arg::with_name("containers")
        .alias("containers")
        .long("containers")
        .short("c")
        .min_values(1)
        .takes_value(true)
        .multiple(true)
        .possible_values(&Provider::variants())
        .help("Set the containers to be provided for docker"),
      Arg::with_name("name")
        .alias("name")
        .long("name")
        .short("n")
        .required(false)
        .takes_value(true)
        .help("Set the file name to use for the config"),
    ])
}

fn migration_subcommand<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("migrate")
    .about(
      "A group of commands for generating, running, and reverting \
             migrations.",
    )
    .setting(AppSettings::VersionlessSubcommands)
    .arg(connection_arg())
    .subcommand(
      SubCommand::with_name("save").about("Save the details of the latest changes in the schema in a migration file which would then be applied using the `up` command"),
    )
    .subcommand(
      SubCommand::with_name("up").about("Run all pending migration and create the tables"),
    )
    .subcommand(
      SubCommand::with_name("reset")
        .about("Rollback all application migration and empty the migration directory"),
    )
    .subcommand(
      SubCommand::with_name("rollback")
        .about("Reverts the latest migration operation(s)")
        .arg(
          Arg::with_name("step")
            .help("Specify the number of steps to rollback")
            .required(false),
        ),
    )
    .subcommand(
      SubCommand::with_name("fresh")
        .about("Reset all migrations and run again")
        .arg(
          Arg::with_name("seed")
            .help("Should run the seeds after migrations")
            .required(false),
        ),
    )
    .subcommand(SubCommand::with_name("list").about(
      "Lists all available migrations, marking those that have been applied to the database.",
    ))
    .subcommand(
      SubCommand::with_name("pending").about("Returns true if there are any pending migrations."),
    )
    .setting(AppSettings::SubcommandRequiredElseHelp)
}

fn seed_subcommand<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("seed")
    .about("A group of commands for generating, running seedings.")
    .setting(AppSettings::VersionlessSubcommands)
    .arg(connection_arg())
    .subcommand(
      SubCommand::with_name("up").about("Run all seedings").arg(
        Arg::with_name("name")
          .help("Specify the name of the seed to run")
          .required(false),
      ),
    )
    .subcommand(
      SubCommand::with_name("create")
        .about("Create a new seeding file")
        .arg(
          Arg::with_name("name")
            .help("Specify the name of the seeding file")
            .required(true),
        ),
    )
    .subcommand(SubCommand::with_name("list").about("List all the available seedings"))
    .setting(AppSettings::SubcommandRequiredElseHelp)
}

fn completions_subcommand<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("completions")
    .about("Generate shell completion scripts for the spectre command.")
    .setting(AppSettings::VersionlessSubcommands)
    .arg(
      Arg::with_name("shell")
        .short("s")
        .alias("shell")
        .long("shell")
        .takes_value(true)
        .possible_values(&Shell::variants()),
    )
    .setting(AppSettings::ArgRequiredElseHelp)
}
