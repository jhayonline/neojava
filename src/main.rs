use anyhow::{Context, Result};
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

    /// Create a new Spring Boot project
    NewSpring {
        /// Name of the project (artifactId)
        name: String,

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

    /// Spring Boot commands
    Spring {
        #[command(subcommand)]
        command: SpringCommands,
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

    /// List available templates
    List,
}

#[derive(Subcommand)]
enum SpringCommands {
    /// Run Spring Boot application
    Run {
        /// Path to Spring Boot project (default: current directory)
        #[arg(short, long, default_value = ".")]
        path: String,
    },

    /// Compile Spring Boot application (mvn clean compile)
    Compile {
        /// Path to Spring Boot project (default: current directory)
        #[arg(short, long, default_value = ".")]
        path: String,
    },

    /// Package Spring Boot application (mvn clean package)
    Package {
        /// Path to Spring Boot project (default: current directory)
        #[arg(short, long, default_value = ".")]
        path: String,
    },

    /// Create JPA Entity
    Entity { name: String },

    /// Create DTO (Data Transfer Object)
    Dto { name: String },

    /// Create MapStruct Mapper
    Mapper { name: String },

    /// Create Repository interface
    Repository { name: String },

    /// Create Service with implementation
    Impl { name: String },

    /// Create REST Controller
    RestController { name: String },

    /// Create Exception class
    Exception { name: String },
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
        Commands::NewSpring { name, path } => {
            create_spring_project(&name, &path)?;
        }
        Commands::Run { file, args } => {
            run_java_file(&file, &args)?;
        }
        Commands::Spring { command } => match command {
            SpringCommands::Run { path } => {
                run_spring_app(&path)?;
            }
            SpringCommands::Compile { path } => {
                compile_spring_app(&path)?;
            }
            SpringCommands::Package { path } => {
                package_spring_app(&path)?;
            }
            SpringCommands::Entity { name } => {
                create_entity(&name)?;
            }
            SpringCommands::Dto { name } => {
                create_dto(&name)?;
            }
            SpringCommands::Mapper { name } => {
                create_mapper(&name)?;
            }
            SpringCommands::Repository { name } => {
                create_repository(&name)?;
            }
            SpringCommands::Impl { name } => {
                create_service_impl(&name)?;
            }
            SpringCommands::RestController { name } => {
                create_rest_controller(&name)?;
            }
            SpringCommands::Exception { name } => {
                create_exception(&name)?;
            }
        },
        Commands::Make {
            file_type,
            name,
            path,
            package,
            main,
        } => {
            create_java_file(&file_type, &name, &path, package.as_deref(), main)?;
        }
        Commands::List => {
            list_templates();
        }
    }

    Ok(())
}

fn create_spring_project(name: &str, path: &str) -> Result<()> {
    let project_path = Path::new(path).join(name);

    if project_path.exists() {
        anyhow::bail!("Project directory already exists: {:?}", project_path);
    }

    println!("🚀 Creating Spring Boot project: {}", name);
    println!();
    println!("📦 Enter dependencies (comma-separated, default: web):");
    print!("> ");
    std::io::Write::flush(&mut std::io::stdout())?;

    let mut deps = String::new();
    std::io::stdin().read_line(&mut deps)?;
    let deps = deps.trim();
    let deps = if deps.is_empty() { "web" } else { deps };

    println!();
    println!("🏷️  Enter groupId (default: dev.jhayonline):");
    print!("> ");
    std::io::Write::flush(&mut std::io::stdout())?;

    let mut group_id = String::new();
    std::io::stdin().read_line(&mut group_id)?;
    let group_id = group_id.trim();
    let group_id = if group_id.is_empty() {
        "dev.jhayonline"
    } else {
        group_id
    };

    println!();
    println!("📁 Creating project with:");
    println!("   GroupId: {}", group_id);
    println!("   ArtifactId: {}", name);
    println!("   Dependencies: {}", deps);
    println!();

    // Use curl to download Spring Boot project
    let url = format!(
        "https://start.spring.io/starter.zip?\
         dependencies={}&\
         groupId={}&\
         artifactId={}&\
         name={}&\
         packageName={}.{}&\
         javaVersion=21&\
         type=maven-project",
        deps,
        group_id,
        name,
        name,
        group_id,
        name.replace("-", "")
    );

    println!("📥 Downloading project...");

    let output = ProcessCommand::new("curl")
        .arg("-L")
        .arg("-o")
        .arg(format!("{}.zip", name))
        .arg(&url)
        .current_dir(path)
        .output()
        .context("Failed to download Spring Boot project. Make sure curl is installed.")?;

    if !output.status.success() {
        anyhow::bail!("Failed to download project");
    }

    println!("📦 Extracting...");

    let extract_status = ProcessCommand::new("unzip")
        .arg("-q")
        .arg(format!("{}.zip", name))
        .arg("-d")
        .arg(name)
        .current_dir(path)
        .status()
        .context("Failed to extract project. Make sure unzip is installed.")?;

    if !extract_status.success() {
        anyhow::bail!("Failed to extract project");
    }

    // Clean up zip file
    fs::remove_file(Path::new(path).join(format!("{}.zip", name)))?;

    println!();
    println!("✅ Spring Boot project created successfully!");
    println!();
    println!("Next steps:");
    println!("  cd {}", name);
    println!("  neojava spring run");
    println!();
    println!("Or generate components:");
    println!("  neojava spring entity User");
    println!("  neojava spring rest-controller User");

    Ok(())
}

fn run_spring_app(path: &str) -> Result<()> {
    let project_path = Path::new(path);

    // Check if pom.xml exists
    if !project_path.join("pom.xml").exists() {
        anyhow::bail!("No pom.xml found. Are you in a Spring Boot project?");
    }

    println!("🚀 Starting Spring Boot application...");
    println!();

    let status = ProcessCommand::new("mvn")
        .arg("spring-boot:run")
        .current_dir(project_path)
        .status()
        .context("Failed to run Spring Boot application")?;

    if !status.success() {
        anyhow::bail!("Application stopped with error");
    }

    Ok(())
}

fn compile_spring_app(path: &str) -> Result<()> {
    let project_path = Path::new(path);

    if !project_path.join("pom.xml").exists() {
        anyhow::bail!("No pom.xml found. Are you in a Spring Boot project?");
    }

    println!("🔨 Compiling Spring Boot application...");

    let status = ProcessCommand::new("mvn")
        .arg("clean")
        .arg("compile")
        .current_dir(project_path)
        .status()
        .context("Failed to compile")?;

    if status.success() {
        println!("✅ Compilation successful!");
    } else {
        anyhow::bail!("Compilation failed");
    }

    Ok(())
}

fn package_spring_app(path: &str) -> Result<()> {
    let project_path = Path::new(path);

    if !project_path.join("pom.xml").exists() {
        anyhow::bail!("No pom.xml found. Are you in a Spring Boot project?");
    }

    println!("📦 Packaging Spring Boot application...");

    let status = ProcessCommand::new("mvn")
        .arg("clean")
        .arg("package")
        .current_dir(project_path)
        .status()
        .context("Failed to package")?;

    if status.success() {
        println!("✅ Package created successfully!");
    } else {
        anyhow::bail!("Packaging failed");
    }

    Ok(())
}

fn create_entity(name: &str) -> Result<()> {
    let content = format!(
        r#"package com.example.demo;

import jakarta.persistence.*;
import lombok.Data;
import lombok.NoArgsConstructor;
import lombok.AllArgsConstructor;

@Entity
@Table(name = "{}s")
@Data
@NoArgsConstructor
@AllArgsConstructor
public class {} {{

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;

    @Column(nullable = false)
    private String name;

    // Add your fields here

}}
"#,
        name.to_lowercase(),
        name
    );

    write_java_file(name, &content, "entity")
}

fn create_dto(name: &str) -> Result<()> {
    let content = format!(
        r#"package com.example.demo.dto;

import lombok.Data;
import lombok.NoArgsConstructor;
import lombok.AllArgsConstructor;

@Data
@NoArgsConstructor
@AllArgsConstructor
public class {}DTO {{

    private Long id;
    private String name;
    // Add your fields here

}}
"#,
        name
    );

    write_java_file(&format!("{}DTO", name), &content, "dto")
}

fn create_mapper(name: &str) -> Result<()> {
    let content = format!(
        r#"package com.example.demo.mapper;

import com.example.demo.dto.{}DTO;
import com.example.demo.entity.{}; 
import org.mapstruct.Mapper;
import org.mapstruct.factory.Mappers;
import java.util.List;

@Mapper(componentModel = "spring")
public interface {}Mapper {{

    {}Mapper INSTANCE = Mappers.getMapper({}Mapper.class);

    {}DTO toDto({} entity);
    {} toEntity({}DTO dto);
    List<{}DTO> toDtoList(List<{}> entities);

}}
"#,
        name, name, name, name, name, name, name, name, name, name, name
    );

    write_java_file(&format!("{}Mapper", name), &content, "mapper")
}

fn create_repository(name: &str) -> Result<()> {
    let content = format!(
        r#"package com.example.demo.repository;

import com.example.demo.entity.{}; 
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface {}Repository extends JpaRepository<{}, Long> {{
    // Add custom query methods here
}}
"#,
        name, name, name
    );

    write_java_file(&format!("{}Repository", name), &content, "repository")
}

fn create_service_impl(name: &str) -> Result<()> {
    // Create interface
    let interface_content = format!(
        r#"package com.example.demo.service;

import com.example.demo.dto.{}DTO;
import java.util.List;

public interface {}Service {{
    List<{}DTO> findAll();
    {}DTO findById(Long id);
    {}DTO save({}DTO dto);
    {}DTO update(Long id, {}DTO dto);
    void delete(Long id);
}}
"#,
        name, name, name, name, name, name, name, name
    );

    // Create implementation - FIXED: all placeholders properly aligned
    let impl_content = format!(
        r#"package com.example.demo.service.impl;

import com.example.demo.dto.{}DTO;
import com.example.demo.entity.{};
import com.example.demo.mapper.{}Mapper;
import com.example.demo.repository.{}Repository;
import com.example.demo.service.{}Service;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import java.util.List;

@Service
@RequiredArgsConstructor
@Transactional
public class {}ServiceImpl implements {}Service {{

    private final {}Repository repository;
    private final {}Mapper mapper;

    @Override
    public List<{}DTO> findAll() {{
        return mapper.toDtoList(repository.findAll());
    }}

    @Override
    public {}DTO findById(Long id) {{
        {} entity = repository.findById(id)
            .orElseThrow(() -> new RuntimeException("{} not found with id: " + id));
        return mapper.toDto(entity);
    }}

    @Override
    public {}DTO save({}DTO dto) {{
        {} entity = mapper.toEntity(dto);
        {} savedEntity = repository.save(entity);
        return mapper.toDto(savedEntity);
    }}

    @Override
    public {}DTO update(Long id, {}DTO dto) {{
        {} existingEntity = repository.findById(id)
            .orElseThrow(() -> new RuntimeException("{} not found with id: " + id));
        existingEntity.setName(dto.getName());
        // Update other fields here
        {} updatedEntity = repository.save(existingEntity);
        return mapper.toDto(updatedEntity);
    }}

    @Override
    public void delete(Long id) {{
        repository.deleteById(id);
    }}
}}
"#,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
        name,
    );

    // Create service directory if needed
    let service_path = Path::new("src/main/java/com/example/demo/service");
    fs::create_dir_all(service_path)?;
    let impl_path = Path::new("src/main/java/com/example/demo/service/impl");
    fs::create_dir_all(impl_path)?;

    let interface_file =
        Path::new("src/main/java/com/example/demo/service").join(format!("{}Service.java", name));
    let impl_file = Path::new("src/main/java/com/example/demo/service/impl")
        .join(format!("{}ServiceImpl.java", name));

    fs::write(interface_file, interface_content)?;
    fs::write(impl_file, impl_content)?;

    println!("✅ Created Service: {}Service", name);
    println!("✅ Created Service Impl: {}ServiceImpl", name);

    Ok(())
}

fn create_rest_controller(name: &str) -> Result<()> {
    let lower_name = name.to_lowercase();
    let content = format!(
        r#"package com.example.demo.controller;

import com.example.demo.dto.{}DTO;
import com.example.demo.service.{}Service;
import lombok.RequiredArgsConstructor;
import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.*;
import java.util.List;

@RestController
@RequestMapping("/api/{}")
@RequiredArgsConstructor
public class {}Controller {{

    private final {}Service service;

    @GetMapping
    public List<{}DTO> getAll() {{
        return service.findAll();
    }}

    @GetMapping("/{{id}}")
    public {}DTO getById(@PathVariable Long id) {{
        return service.findById(id);
    }}

    @PostMapping
    @ResponseStatus(HttpStatus.CREATED)
    public {}DTO create(@RequestBody {}DTO dto) {{
        return service.save(dto);
    }}

    @PutMapping("/{{id}}")
    public {}DTO update(@PathVariable Long id, @RequestBody {}DTO dto) {{
        return service.update(id, dto);
    }}

    @DeleteMapping("/{{id}}")
    @ResponseStatus(HttpStatus.NO_CONTENT)
    public void delete(@PathVariable Long id) {{
        service.delete(id);
    }}
}}
"#,
        name, name, lower_name, name, name, name, name, name, name, name, name
    );

    write_java_file(&format!("{}Controller", name), &content, "controller")
}

fn create_exception(name: &str) -> Result<()> {
    let content = format!(
        r#"package com.example.demo.exception;

public class {}Exception extends RuntimeException {{

    public {}Exception(String message) {{
        super(message);
    }}

    public {}Exception(String message, Throwable cause) {{
        super(message, cause);
    }}
}}
"#,
        name, name, name
    );

    write_java_file(name, &content, "exception")
}

fn write_java_file(name: &str, content: &str, subdir: &str) -> Result<()> {
    let base_path = Path::new("src/main/java/com/example/demo");
    let dir_path = base_path.join(subdir);
    fs::create_dir_all(&dir_path)?;

    let file_path = dir_path.join(format!("{}.java", name));

    if file_path.exists() {
        anyhow::bail!("File already exists: {:?}", file_path);
    }

    let mut file = fs::File::create(&file_path)?;
    file.write_all(content.as_bytes())?;

    println!("✅ Created {}: {}", subdir, file_path.display());

    Ok(())
}

fn run_java_file(file: &str, args: &[String]) -> Result<()> {
    let file_name = if file.ends_with(".java") {
        file.to_string()
    } else {
        format!("{}.java", file)
    };

    let class_name = file_name.trim_end_matches(".java");

    if !Path::new(&file_name).exists() {
        anyhow::bail!("File not found: {}", file_name);
    }

    println!("🔨 Compiling {}...", file_name);

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

    let src_path = project_path.join("src/main/java");
    let test_path = project_path.join("src/test/java");
    fs::create_dir_all(&src_path)?;
    fs::create_dir_all(&test_path)?;

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
    let name_valid = regex::Regex::new(r"^[A-Z][a-zA-Z0-9]*$").unwrap();
    if !name_valid.is_match(name) {
        anyhow::bail!("Name must start with uppercase and contain only letters/numbers");
    }

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

fn generate_java_content(
    file_type: &JavaType,
    name: &str,
    package: Option<&str>,
    add_main: bool,
) -> String {
    let package_decl = match package {
        Some(pkg) => format!("package {};\n\n", pkg),
        None => String::new(),
    };

    let body = match file_type {
        JavaType::Class => generate_class(name, add_main),
        JavaType::Interface => generate_interface(name),
        JavaType::Enum => generate_enum(name),
        JavaType::Record => generate_record(name),
        JavaType::AbstractClass => generate_abstract_class(name),
        JavaType::SealedClass => generate_sealed_class(name),
    };

    format!("{}{}", package_decl, body)
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

fn list_templates() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║                        📦 NEOJAVA CLI                          ║");
    println!("║              Java & Spring Boot Project Generator              ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();

    println!("🆕 CREATE NEW PROJECTS");
    println!("────────────────────────────────────────────────────────────────");
    println!("  neojava new <name>                           Plain Java Maven project");
    println!("  neojava new spring <name>                    Spring Boot project (interactive)");
    println!();

    println!("🍃 SPRING BOOT COMMANDS (run inside project)");
    println!("────────────────────────────────────────────────────────────────");
    println!("  neojava spring run                           Start Spring Boot application");
    println!("  neojava spring compile                       Run mvn clean compile");
    println!("  neojava spring package                       Run mvn clean package");
    println!();

    println!("📁 GENERATE SPRING COMPONENTS");
    println!("────────────────────────────────────────────────────────────────");
    println!("  neojava spring entity <Name>                 @Entity JPA class with Lombok");
    println!("  neojava spring dto <Name>                    Data Transfer Object with Lombok");
    println!("  neojava spring mapper <Name>                 MapStruct mapper interface");
    println!("  neojava spring repository <Name>             JPA repository interface");
    println!("  neojava spring impl <Name>                   Service interface + implementation");
    println!("  neojava spring rest-controller <Name>        @RestController with CRUD endpoints");
    println!("  neojava spring exception <Name>              Custom runtime exception");
    println!();

    println!("📁 GENERATE REGULAR JAVA FILES");
    println!("────────────────────────────────────────────────────────────────");
    println!("  neojava make class <Name>                    Regular Java class");
    println!("  neojava make class <Name> --main             Java class with main() method");
    println!("  neojava make interface <Name>                Java interface");
    println!("  neojava make enum <Name>                     Java enum");
    println!("  neojava make record <Name>                   Java record");
    println!("  neojava make abstract-class <Name>           Abstract class");
    println!("  neojava make sealed-class <Name>             Sealed class");
    println!();

    println!("🏃 RUN JAVA FILES");
    println!("────────────────────────────────────────────────────────────────");
    println!("  neojava run <file>                           Compile and run .java file");
    println!("  neojava run Main.java                        Run with .java extension");
    println!("  neojava run Main                             Run without extension");
    println!("  neojava run Main.java --arg1 --arg2          Pass arguments to program");
    println!();

    println!("ℹ️  OTHER COMMANDS");
    println!("────────────────────────────────────────────────────────────────");
    println!("  neojava list                                 Show this help menu");
    println!("  neojava --help                               Show CLI help");
    println!("  neojava --version                            Show version");
    println!();

    println!("📝 OPTIONS");
    println!("────────────────────────────────────────────────────────────────");
    println!("  -p, --path <PATH>      Directory to create file/project (default: .)");
    println!("  --package <PKG>        Package declaration (e.g., com.example.models)");
    println!("  --group-id <ID>        Maven group ID for new projects (default: com.example)");
    println!("  --main                 Add main method (class only)");
    println!();

    println!("💡 EXAMPLES");
    println!("────────────────────────────────────────────────────────────────");
    println!("  # Create and run a Spring Boot project");
    println!("  neojava new spring myapi");
    println!("  cd myapi");
    println!("  neojava spring entity User");
    println!("  neojava spring rest-controller User");
    println!("  neojava spring run");
    println!();
    println!("  # Create a plain Java project");
    println!("  neojava new myapp --group-id dev.jhayonline");
    println!("  cd myapp");
    println!("  neojava make class Main --main");
    println!("  neojava run Main.java");
    println!();
    println!("  # Generate Spring components in existing project");
    println!("  neojava spring dto Product");
    println!("  neojava spring mapper Product");
    println!("  neojava spring repository Product");
    println!("  neojava spring impl Product");
    println!("  neojava spring rest-controller Product");
    println!();
    println!("  # Generate plain Java files");
    println!("  neojava make class UserService --package com.example.service");
    println!("  neojava make interface CrudRepository --package com.example.repository");
    println!("  neojava make enum Status --package com.example.models");
    println!();
    println!("  # Run Java files");
    println!("  neojava run Hello.java");
    println!("  neojava run Calculator 5 + 3");
    println!();

    println!("⚡ QUICK TIPS");
    println!("────────────────────────────────────────────────────────────────");
    println!("  • All class/component names must start with UPPERCASE");
    println!("  • Spring components are generated in standard package structure");
    println!("  • DTO, Entity, Mapper work together for full CRUD");
    println!("  • Service impl creates both interface and implementation");
    println!("  • Run `neojava spring run` from inside any Spring Boot project");
    println!();
}
