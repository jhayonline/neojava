use anyhow::{Context, Result};
use chrono::Local;
use clap::{Parser, Subcommand};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command as ProcessCommand;

/// Java file generator for Neovim
#[derive(Parser)]
#[command(name = "neojava")]
#[command(about = "Generate Java class files with boilerplate", long_about = None)]
#[command(version = "0.3.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Java project
    New {
        /// Name of the project
        name: String,

        /// Group ID for Maven (default: com.example)
        #[arg(short, long, default_value = "com.example")]
        group_id: String,

        /// Path where to create the project (default: current directory)
        #[arg(short, long, default_value = ".")]
        path: String,
    },

    /// Run a Java file (compile + execute)
    Run {
        /// Name of the Java file (with or without .java extension)
        file: String,

        /// Arguments to pass to the Java program
        #[arg(last = true)]
        args: Vec<String>,
    },

    /// Create a Java file
    Make {
        /// Type of file to create
        #[arg(value_enum)]
        file_type: JavaType,

        /// Name of the file
        name: String,

        /// Path where to create the file (default: current directory)
        #[arg(short, long, default_value = ".")]
        path: String,

        /// Package name (optional)
        #[arg(short, long)]
        package: Option<String>,

        /// Add main method (only for classes)
        #[arg(long)]
        main: bool,
    },

    /// Create a Spring Boot file
    Spring {
        /// Type of Spring Boot file to create
        #[arg(value_enum)]
        spring_type: SpringType,

        /// Name of the file (e.g., User for UserController)
        name: String,

        /// Path where to create the file (default: current directory)
        #[arg(short, long, default_value = ".")]
        path: String,

        /// Package name (optional, defaults to com.example.{type})
        #[arg(short, long)]
        package: Option<String>,
    },

    /// List available templates
    List,
}

#[derive(clap::ValueEnum, Clone)]
enum JavaType {
    Class,
    Interface,
    Enum,
    Record,
    AbstractClass,
    SealedClass,
}

#[derive(clap::ValueEnum, Clone)]
enum SpringType {
    Controller,
    Service,
    Repository,
    Configuration,
    Component,
    RestController,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New {
            name,
            group_id,
            path,
        } => {
            create_java_project(&name, &group_id, &path)?;
        }
        Commands::Run { file, args } => {
            run_java_file(&file, &args)?;
        }
        Commands::Make {
            file_type,
            name,
            path,
            package,
            main,
        } => {
            create_java_file(&file_type, &name, &path, package.as_deref(), main)?;
        }
        Commands::Spring {
            spring_type,
            name,
            path,
            package,
        } => {
            create_spring_file(&spring_type, &name, &path, package.as_deref())?;
        }
        Commands::List => {
            list_templates();
        }
    }

    Ok(())
}

fn run_java_file(file: &str, args: &[String]) -> Result<()> {
    // Handle file name with or without .java extension
    let file_name = if file.ends_with(".java") {
        file.to_string()
    } else {
        format!("{}.java", file)
    };

    let class_name = file_name.trim_end_matches(".java");

    // Check if file exists
    if !Path::new(&file_name).exists() {
        anyhow::bail!("File not found: {}", file_name);
    }

    println!("🔨 Compiling {}...", file_name);

    // Compile the Java file
    let compile_status = ProcessCommand::new("javac")
        .arg(&file_name)
        .status()
        .context("Failed to run javac. Make sure Java JDK is installed.")?;

    if !compile_status.success() {
        anyhow::bail!("Compilation failed");
    }

    println!("✅ Compilation successful!");
    println!("🚀 Running {}...", class_name);
    println!("{}", "=".repeat(50));

    // Run the compiled class
    let run_status = ProcessCommand::new("java")
        .arg(class_name)
        .args(args)
        .status()
        .context("Failed to run java")?;

    if !run_status.success() {
        anyhow::bail!("Execution failed");
    }

    Ok(())
}

fn create_java_project(name: &str, group_id: &str, path: &str) -> Result<()> {
    let project_path = Path::new(path).join(name);

    if project_path.exists() {
        anyhow::bail!("Project directory already exists: {:?}", project_path);
    }

    println!("🚀 Creating Java project: {}", name);

    // Create directory structure
    let src_path = project_path.join("src/main/java");
    let test_path = project_path.join("src/test/java");
    fs::create_dir_all(&src_path)?;
    fs::create_dir_all(&test_path)?;

    // Create pom.xml
    let pom_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 
         http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    
    <groupId>{}</groupId>
    <artifactId>{}</artifactId>
    <version>1.0-SNAPSHOT</version>
    <packaging>jar</packaging>
    
    <properties>
        <maven.compiler.source>21</maven.compiler.source>
        <maven.compiler.target>21</maven.compiler.target>
        <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
    </properties>
    
    <dependencies>
        <dependency>
            <groupId>org.junit.jupiter</groupId>
            <artifactId>junit-jupiter</artifactId>
            <version>5.10.0</version>
            <scope>test</scope>
        </dependency>
    </dependencies>
    
    <build>
        <plugins>
            <plugin>
                <groupId>org.apache.maven.plugins</groupId>
                <artifactId>maven-compiler-plugin</artifactId>
                <version>3.11.0</version>
            </plugin>
            <plugin>
                <groupId>org.codehaus.mojo</groupId>
                <artifactId>exec-maven-plugin</artifactId>
                <version>3.1.0</version>
                <configuration>
                    <mainClass>{}.Main</mainClass>
                </configuration>
            </plugin>
        </plugins>
    </build>
</project>
"#,
        group_id, name, group_id
    );

    let pom_path = project_path.join("pom.xml");
    let mut pom_file = fs::File::create(&pom_path)?;
    pom_file.write_all(pom_content.as_bytes())?;

    // Create .gitignore
    let gitignore_content = r#"target/
*.class
.settings/
.project
.classpath
.DS_Store
"#;
    let gitignore_path = project_path.join(".gitignore");
    let mut gitignore_file = fs::File::create(&gitignore_path)?;
    gitignore_file.write_all(gitignore_content.as_bytes())?;

    println!("  ✓ Created Maven project structure");
    println!("  ✓ Generated pom.xml");
    println!("  ✓ Created .gitignore");
    println!(
        "\n✅ Project created successfully at: {}",
        project_path.display()
    );
    println!("\nNext steps:");
    println!("  cd {}", name);
    println!("  neojava make class Main --main");
    println!("\nBuild with:");
    println!("  mvn clean compile");
    println!("  mvn exec:java -Dexec.mainClass={}.Main", group_id);

    Ok(())
}

fn create_java_file(
    file_type: &JavaType,
    name: &str,
    path: &str,
    package: Option<&str>,
    add_main: bool,
) -> Result<()> {
    // Validate class name
    let name_valid = regex::Regex::new(r"^[A-Z][a-zA-Z0-9]*$").unwrap();
    if !name_valid.is_match(name) {
        anyhow::bail!("Name must start with uppercase and contain only letters/numbers");
    }

    // Build the full file path
    let base_path = Path::new(path);
    let package_path = match package {
        Some(pkg) => {
            let pkg_path_str = pkg.replace('.', "/");
            let pkg_path = Path::new(&pkg_path_str);
            base_path.join(pkg_path)
        }
        None => base_path.to_path_buf(),
    };

    fs::create_dir_all(&package_path)
        .with_context(|| format!("Failed to create directory: {:?}", package_path))?;

    let file_path = package_path.join(format!("{}.java", name));

    if file_path.exists() {
        anyhow::bail!("File already exists: {:?}", file_path);
    }

    let content = generate_java_content(file_type, name, package, add_main);

    let mut file = fs::File::create(&file_path)?;
    file.write_all(content.as_bytes())?;

    println!(
        "✅ Created {}: {}",
        match file_type {
            JavaType::Class => "Class",
            JavaType::Interface => "Interface",
            JavaType::Enum => "Enum",
            JavaType::Record => "Record",
            JavaType::AbstractClass => "Abstract Class",
            JavaType::SealedClass => "Sealed Class",
        },
        file_path.display()
    );

    Ok(())
}

fn create_spring_file(
    spring_type: &SpringType,
    name: &str,
    path: &str,
    package: Option<&str>,
) -> Result<()> {
    // Validate name
    let name_valid = regex::Regex::new(r"^[A-Z][a-zA-Z0-9]*$").unwrap();
    if !name_valid.is_match(name) {
        anyhow::bail!("Name must start with uppercase and contain only letters/numbers");
    }

    let package_path = match package {
        Some(pkg) => {
            let pkg_path_str = pkg.replace('.', "/");
            Path::new(path).join(pkg_path_str)
        }
        None => {
            let default_pkg = match spring_type {
                SpringType::Controller => "com.example.controllers",
                SpringType::RestController => "com.example.controllers",
                SpringType::Service => "com.example.services",
                SpringType::Repository => "com.example.repositories",
                SpringType::Configuration => "com.example.config",
                SpringType::Component => "com.example.components",
            };
            let pkg_path_str = default_pkg.replace('.', "/");
            Path::new(path).join(pkg_path_str)
        }
    };

    fs::create_dir_all(&package_path)
        .with_context(|| format!("Failed to create directory: {:?}", package_path))?;

    let file_name = match spring_type {
        SpringType::Controller => format!("{}Controller.java", name),
        SpringType::RestController => format!("{}Controller.java", name),
        SpringType::Service => format!("{}Service.java", name),
        SpringType::Repository => format!("{}Repository.java", name),
        SpringType::Configuration => format!("{}Config.java", name),
        SpringType::Component => format!("{}.java", name),
    };

    let file_path = package_path.join(&file_name);

    if file_path.exists() {
        anyhow::bail!("File already exists: {:?}", file_path);
    }

    let content = generate_spring_content(spring_type, name, package);

    let mut file = fs::File::create(&file_path)?;
    file.write_all(content.as_bytes())?;

    println!(
        "✅ Created Spring Boot {}: {}",
        match spring_type {
            SpringType::Controller => "Controller",
            SpringType::RestController => "REST Controller",
            SpringType::Service => "Service",
            SpringType::Repository => "Repository",
            SpringType::Configuration => "Configuration",
            SpringType::Component => "Component",
        },
        file_path.display()
    );

    Ok(())
}

fn generate_java_content(
    file_type: &JavaType,
    name: &str,
    package: Option<&str>,
    add_main: bool,
) -> String {
    let date = Local::now().format("%Y-%m-%d");
    let package_decl = match package {
        Some(pkg) => format!("package {};\n\n", pkg),
        None => String::new(),
    };

    let header = format!("/*\n * {}.java\n * Created on {}\n */\n\n", name, date);

    let body = match file_type {
        JavaType::Class => generate_class(name, add_main),
        JavaType::Interface => generate_interface(name),
        JavaType::Enum => generate_enum(name),
        JavaType::Record => generate_record(name),
        JavaType::AbstractClass => generate_abstract_class(name),
        JavaType::SealedClass => generate_sealed_class(name),
    };

    format!("{}{}{}", header, package_decl, body)
}

fn generate_class(name: &str, add_main: bool) -> String {
    let mut content = format!("public class {} {{\n", name);

    if add_main {
        content.push_str(
            r#"    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }
"#,
        );
    } else {
        content.push_str("    // Constructor\n");
        content.push_str(&format!("    public {}() {{\n", name));
        content.push_str("        // TODO: Initialize\n");
        content.push_str("    }\n\n");
        content.push_str("    // Add your methods here\n");
    }

    content.push_str("}\n");
    content
}

fn generate_interface(name: &str) -> String {
    format!(
        "public interface {} {{\n\n    // TODO: Add interface methods\n\n}}\n",
        name
    )
}

fn generate_enum(name: &str) -> String {
    format!(
        "public enum {} {{\n    // TODO: Add enum constants\n\n    // TODO: Add fields and methods if needed\n\n}}\n",
        name
    )
}

fn generate_record(name: &str) -> String {
    format!(
        "public record {}(
    // TODO: Define record components
    // For example: String name, int age
) {{\n\n    // TODO: Add compact constructor or methods if needed\n\n}}\n",
        name
    )
}

fn generate_abstract_class(name: &str) -> String {
    format!(
        "public abstract class {} {{\n\n    // TODO: Add abstract methods\n    // TODO: Add concrete methods\n\n}}\n",
        name
    )
}

fn generate_sealed_class(name: &str) -> String {
    format!(
        "public sealed class {} permits {{\n    // TODO: Add subclasses\n\n    // TODO: Add methods\n\n}}\n",
        name
    )
}

fn generate_spring_content(spring_type: &SpringType, name: &str, package: Option<&str>) -> String {
    let date = Local::now().format("%Y-%m-%d");
    let package_decl = match package {
        Some(pkg) => format!("package {};\n\n", pkg),
        None => {
            let default_pkg = match spring_type {
                SpringType::Controller => "com.example.controllers",
                SpringType::RestController => "com.example.controllers",
                SpringType::Service => "com.example.services",
                SpringType::Repository => "com.example.repositories",
                SpringType::Configuration => "com.example.config",
                SpringType::Component => "com.example.components",
            };
            format!("package {};\n\n", default_pkg)
        }
    };

    let header = format!(
        "/*\n * {}{}.java\n * Created on {}\n */\n\n",
        name,
        match spring_type {
            SpringType::Controller => "Controller",
            SpringType::RestController => "Controller",
            SpringType::Service => "Service",
            SpringType::Repository => "Repository",
            SpringType::Configuration => "Config",
            SpringType::Component => "",
        },
        date
    );

    let imports = match spring_type {
        SpringType::Controller => {
            "import org.springframework.web.bind.annotation.*;\nimport java.util.List;\n"
        }
        SpringType::RestController => {
            "import org.springframework.web.bind.annotation.*;\nimport java.util.List;\n"
        }
        SpringType::Service => "import org.springframework.stereotype.Service;\n",
        SpringType::Repository => "import org.springframework.stereotype.Repository;\n",
        SpringType::Configuration => {
            "import org.springframework.context.annotation.Configuration;\nimport org.springframework.context.annotation.Bean;\n"
        }
        SpringType::Component => "import org.springframework.stereotype.Component;\n",
    };

    let body = match spring_type {
        SpringType::Controller => generate_controller(name),
        SpringType::RestController => generate_rest_controller(name),
        SpringType::Service => generate_service(name),
        SpringType::Repository => generate_repository(name),
        SpringType::Configuration => generate_configuration(name),
        SpringType::Component => generate_component(name),
    };

    format!("{}{}{}{}", header, package_decl, imports, body)
}

fn generate_controller(name: &str) -> String {
    let lower_name = name.to_lowercase();
    format!(
        r#"@Controller
@RequestMapping("/{}")
public class {}Controller {{

    private final {}Service service;

    public {}Controller({}Service service) {{
        this.service = service;
    }}

    @GetMapping
    public String listPage() {{
        return "{}List";
    }}

    @GetMapping("/{{id}}")
    public String viewPage(@PathVariable Long id) {{
        return "{}View";
    }}
}}
"#,
        lower_name, name, name, name, name, lower_name, lower_name
    )
}

fn generate_rest_controller(name: &str) -> String {
    let lower_name = name.to_lowercase();
    format!(
        r#"@RestController
@RequestMapping("/api/{}")
public class {}Controller {{

    private final {}Service service;

    public {}Controller({}Service service) {{
        this.service = service;
    }}

    @GetMapping
    public List<{}> getAll() {{
        return service.findAll();
    }}

    @GetMapping("/{{id}}")
    public {} getById(@PathVariable Long id) {{
        return service.findById(id);
    }}

    @PostMapping
    public {} create(@RequestBody {} entity) {{
        return service.save(entity);
    }}

    @PutMapping("/{{id}}")
    public {} update(@PathVariable Long id, @RequestBody {} entity) {{
        return service.update(id, entity);
    }}

    @DeleteMapping("/{{id}}")
    public void delete(@PathVariable Long id) {{
        service.delete(id);
    }}
}}
"#,
        lower_name, name, name, name, name, name, name, name, name, name, name
    )
}

fn generate_service(name: &str) -> String {
    format!(
        r#"@Service
public class {}Service {{

    private final {}Repository repository;

    public {}Service({}Repository repository) {{
        this.repository = repository;
    }}

    public List<{}> findAll() {{
        return repository.findAll();
    }}

    public {} findById(Long id) {{
        return repository.findById(id).orElseThrow();
    }}

    public {} save({} entity) {{
        return repository.save(entity);
    }}

    public {} update(Long id, {} entity) {{
        entity.setId(id);
        return repository.save(entity);
    }}

    public void delete(Long id) {{
        repository.deleteById(id);
    }}
}}
"#,
        name, name, name, name, name, name, name, name, name, name
    )
}

fn generate_repository(name: &str) -> String {
    format!(
        r#"@Repository
public interface {}Repository extends JpaRepository<{}, Long> {{
    // TODO: Add custom query methods
}}
"#,
        name, name
    )
}

fn generate_configuration(name: &str) -> String {
    let lower_name = name.to_lowercase();
    format!(
        r#"@Configuration
public class {}Config {{

    @Bean
    public String {}Bean() {{
        return new String();
    }}

    // TODO: Add more beans
}}
"#,
        name, lower_name
    )
}

fn generate_component(name: &str) -> String {
    format!(
        r#"@Component
public class {} {{

    // TODO: Add component logic

    public void doSomething() {{
        // TODO: Implement
    }}
}}
"#,
        name
    )
}

fn list_templates() {
    println!("📦 Available Java templates:");
    println!();
    println!("  🏃 Run a Java file:");
    println!("    neojava run <file>");
    println!("    neojava run Main.java");
    println!("    neojava run Main --arg1 --arg2");
    println!();
    println!("  🆕 Create a new project:");
    println!("    neojava new <name>");
    println!("    neojava new <name> --group-id com.mycompany");
    println!("    neojava new <name> --path ./projects");
    println!();
    println!("  📁 Regular Java files:");
    println!("    neojava make class <Name> [OPTIONS]");
    println!("    neojava make interface <Name> [OPTIONS]");
    println!("    neojava make enum <Name> [OPTIONS]");
    println!("    neojava make record <Name> [OPTIONS]");
    println!("    neojava make abstract-class <Name> [OPTIONS]");
    println!("    neojava make sealed-class <Name> [OPTIONS]");
    println!();
    println!("  🍃 Spring Boot files:");
    println!("    neojava spring controller <Name> [OPTIONS]");
    println!("    neojava spring rest-controller <Name> [OPTIONS]");
    println!("    neojava spring service <Name> [OPTIONS]");
    println!("    neojava spring repository <Name> [OPTIONS]");
    println!("    neojava spring configuration <Name> [OPTIONS]");
    println!("    neojava spring component <Name> [OPTIONS]");
    println!();
    println!("Options:");
    println!("  -p, --path <PATH>     Directory to create file/project (default: .)");
    println!("  --package <PKG>       Package declaration");
    println!("  --group-id <ID>       Maven group ID (default: com.example)");
    println!("  --main                Add main method (class only)");
    println!();
    println!("Examples:");
    println!("  # Run a Java file");
    println!("  neojava run Main.java");
    println!("  neojava run MyApp --verbose --config=settings.json");
    println!();
    println!("  # Create a new project");
    println!("  neojava new MyApp");
    println!("  neojava new MyApp --group-id com.mycompany");
    println!("  neojava new MyApp --path ~/projects");
    println!();
    println!("  # Regular Java");
    println!("  neojava make class User --package com.example.models");
    println!("  neojava make class Main --main");
    println!();
    println!("  # Spring Boot");
    println!("  neojava spring rest-controller User");
    println!("  neojava spring service User --package com.example.services");
}
