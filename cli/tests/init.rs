use temp_test::build_project;

#[test]
fn init_cli_project() {
  let p = build_project("init_cli_project")
    .package_name("spectre")
    .build();

  // sanity check
  assert!(p.has_file("Cargo.toml"), "Cargo.toml file does not exist");
  assert!(!p.has_file("spectre.yaml"));

  let result = p.command("init").run();

  assert!(result.is_success(), "Result was unsuccessful {:?}", result);
  assert!(result.stdout().contains("Project created inside"));
  assert!(p.has_file("spectre.yaml"));
}

#[test]
fn init_project_with_name() {
  let p = build_project("init_cli_project_with_name")
    .package_name("spectre")
    .build();

  // sanity check
  assert!(p.has_file("Cargo.toml"), "Cargo.toml file does not exist");
  assert!(!p.has_file("sample.yaml"));
  assert!(!p.has_file("spectre.yaml"));

  let result = p.command("init").arg("--name=sample").run();

  assert!(result.is_success(), "Result was unsuccessful {:?}", result);
  assert!(result.stdout().contains("Project created inside"));
  assert!(p.has_file("sample.yaml"));
}

#[test]
fn init_project_with_docker() {
  let p = build_project("init_with_docker")
    .package_name("spectre")
    .build();

  let result = p
    .command("init")
    .arg("--docker")
    .arg("--containers=postgres")
    .arg("--containers=mysql")
    .run();

  assert!(result.is_success(), "Result was unsuccessful {:?}", result);
  assert!(result.stdout().contains("Docker file has been created"));
  assert!(p.has_file("docker-compose.yml"));
}

#[test]
fn error_when_file_exist() {
  let p = build_project("init_with_error")
    .file(
      "spectre.yaml",
      r#"
    version: "1.0"
    connections:
      - name: "default"
        datasource:
          url: postgresql://postgres:admin@localhost:5432/postgres?schema=public"
          provider: "postgres"
        auto_migrate: false
        logging: false
  "#,
    )
    .package_name("spectre")
    .build();

  // sanity check
  assert!(p.has_file("Cargo.toml"), "Cargo.toml file does not exist");
  assert!(p.has_file("spectre.yaml"));

  let result = p.command("init").run();

  assert!(result.is_err());

  assert!(result
    .stderr()
    .contains("A config file exist with the name"));
}

#[test]
fn error_when_docker_file_exist() {
  let p = build_project("init_with_docker_error")
    .file(
      "docker-compose.yml",
      r#"
    version: "3.8"
  "#,
    )
    .package_name("spectre")
    .build();

  assert!(p.has_file("docker-compose.yml"));

  let result = p.command("init").run();

  assert!(result.is_success());
  assert!(p.has_file("docker-compose.yml"));
}
