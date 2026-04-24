use anyhow::{Context, Result};
use chrono::Local;
use clap::{Parser, Subcommand};
use std::fs;
use std::io::Write;
use std::path::Path;

/// Java file generator for Neovim
#[derive(Parser)]
#[command(name = "neojava")]
#[command(about = "Generate Java class files with boilerplate", long_about = None)]
#[command(version = "0.2.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
        #[arg(value_enum)]
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
    println!("  neojava make class <Name> [OPTIONS]");
    println!("    - Creates a Java class");
    println!("    - Use --main to add main method");
    println!();
    println!("  neojava make interface <Name> [OPTIONS]");
    println!("    - Creates a Java interface");
    println!();
    println!("  neojava make enum <Name> [OPTIONS]");
    println!("    - Creates a Java enum");
    println!();
    println!("  neojava make record <Name> [OPTIONS]");
    println!("    - Creates a Java record");
    println!();
    println!("  neojava make abstract-class <Name> [OPTIONS]");
    println!("    - Creates an abstract class");
    println!();
    println!("  neojava make sealed-class <Name> [OPTIONS]");
    println!("    - Creates a sealed class");
    println!();
    println!("🍃 Spring Boot templates:");
    println!();
    println!("  neojava spring controller <Name> [OPTIONS]");
    println!("    - Creates a Spring MVC Controller");
    println!();
    println!("  neojava spring rest-controller <Name> [OPTIONS]");
    println!("    - Creates a REST Controller with full CRUD");
    println!();
    println!("  neojava spring service <Name> [OPTIONS]");
    println!("    - Creates a Service layer class");
    println!();
    println!("  neojava spring repository <Name> [OPTIONS]");
    println!("    - Creates a Repository interface");
    println!();
    println!("  neojava spring configuration <Name> [OPTIONS]");
    println!("    - Creates a Configuration class");
    println!();
    println!("  neojava spring component <Name> [OPTIONS]");
    println!("    - Creates a Component");
    println!();
    println!("Options:");
    println!("  -p, --path <PATH>     Directory to create file (default: .)");
    println!("  --package <PKG>       Package declaration");
    println!("  --main                Add main method (class only)");
    println!();
    println!("Examples:");
    println!("  # Regular Java");
    println!("  neojava make class User --package com.example.models");
    println!("  neojava make class Application --main");
    println!("  neojava make abstract-class BaseService");
    println!();
    println!("  # Spring Boot");
    println!("  neojava spring rest-controller User");
    println!("  neojava spring service User --package com.example.services");
    println!("  neojava spring repository User");
    println!("  neojava spring configuration Security");
}
