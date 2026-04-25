# neojava

A Rust CLI tool that brings IntelliJ-like file generation to Neovim for Java and Spring Boot development.

## The Problem

I love Neovim. It's fast, lightweight, and infinitely customizable. But coming from IntelliJ IDEA, there was one thing I really missed - the ability to right-click and generate Java classes, interfaces, enums, or Spring Boot components with all the boilerplate already written.

Every time I wanted to create a new class in Neovim, I had to:

1. Manually create the file
2. Write the package declaration
3. Type out the class definition
4. Add constructor, methods, and all that repetitive stuff

It was tedious and broke my flow. I needed a better way.

## The Solution

I thought: "Why not build a simple CLI tool that does this for me?"

I wanted something that:

- Works from both terminal AND Neovim
- Generates proper Java files with boilerplate
- Supports Spring Boot (controllers, services, repositories)
- Creates proper folder structures based on packages
- Doesn't get in my way

So I built `neojava` - a Rust-based CLI tool that generates Java files with sensible defaults.

## Why Rust?

I could have written this in Bash or Python, but:

- Rust gives me a single binary with no dependencies
- It's blazing fast
- Pattern matching and error handling are fantastic for CLI tools

## How I Built It

### Tech Stack

- **clap** - For CLI argument parsing (amazing library!)
- **anyhow** - Simple error handling
- **chrono** - For timestamps in file headers
- **regex** - For validating class names

### Development Process

1. **Started simple** - First version just created basic Java classes
2. **Added options** - Package paths, main method flag, custom directories
3. **Extended to Spring Boot** - Controllers, REST controllers, services, repositories
4. **Added more Java types** - Abstract classes, sealed classes, records
5. **Integrated with Neovim** - Created keymaps so I can press `<leader>jc` and boom - new class!

### The Tricky Parts

Getting string formatting right in Rust was interesting. Raw string literals (`r#"..."#`) became my best friend for multi-line templates.

Path handling with package names (converting `com.example.models` to `com/example/models`) required some string manipulation.

Making sure the tool works from ANY directory and creates proper nested folders took a few iterations.

## Features

### Create New Project

```bash
# Create a new Java project (Maven based)
neojava new MyApp

# Create with custom group ID
neojava new MyApp --group-id com.mycompany

# Create in specific location
neojava new MyApp --path ~/projects

# What gets created:
# - Standard Maven project structure (src/main/java, src/test/java)
# - pom.xml with Java 21 configuration
# - .gitignore file
# - Ready to build with Maven
```

### Create a Spring Boot Project

```bash
# Create new Spring Boot project (interactive)
neojava new spring myapp

# You'll be prompted for:
# - Dependencies (web, data-jpa, lombok, postgresql, etc.)
# - GroupId (default: dev.jhayonline)

# Navigate into project
cd myapp

# Run the application
neojava spring run
```

### Create full CRUD API for User entity

```bash
neojava spring entity User        # JPA Entity
neojava spring dto User           # Data Transfer Object
neojava spring mapper User        # MapStruct mapper
neojava spring repository User    # JPA Repository
neojava spring impl User          # Service with implementation
neojava spring rest-controller User # REST Controller
neojava spring exception UserNotFound # Custom exception
```

### Inside your Spring Boot project

```bash
neojava spring run        # Start the app on port 8080
neojava spring compile    # Compile the project
neojava spring package    # Create JAR file
```

### Regular Java

```bash
# Basic class
neojava make class User

# Class with main method
neojava make class Application --main

# With package structure
neojava make class UserService --package com.example.services

# Abstract class
neojava make abstract-class BaseRepository

# Interface, Enum, Record
neojava make interface Authenticatable
neojava make enum Role
neojava make record Address
```

### Spring Boot

```bash
# REST Controller with full CRUD
neojava spring rest-controller User

# Service layer
neojava spring service User

# Repository interface
neojava spring repository User

# Configuration class
neojava spring configuration Security

# Generic component
neojava spring component EmailService
```

### Neovim Integration

I added keymaps so now in Neovim I can just:

- `<leader>jc` - Create a Java class
- `<leader>ji` - Create an interface
- `<leader>je` - Create an enum
- `<leader>jr` - Create a record
- `<leader>jspc` - Create a REST controller

The tool automatically creates the file in the current buffer's directory and opens it for me.

## Why This Tool Is Useful

**For me personally:**

- No more typing boilerplate. Ever.
- Consistent file structure across all my projects
- One command creates properly formatted Java files
- Works exactly how my brain expects it to

**For other developers:**

- If you're switching from IntelliJ to Neovim, this fills a big gap
- Saves time on repetitive setup
- Ensures consistency across team projects (if everyone uses it)
- You can extend it with your own templates

**The real win:** I stay in my flow state. No context switching to manually create files or copy-paste from old projects. Just `<leader>jc`, type the name, and keep coding.

## Installation

```bash
# Clone and build
git clone git@github.com:jhayonline/neojava.git
cd neojava
cargo build --release
cargo install --path .
```

## Usage Examples

```bash
# Create a Spring Boot REST API in one command
neojava spring rest-controller Product

# This generates:
# - ProductController.java (with all CRUD endpoints)
# - ProductService.java (with business logic)
# - ProductRepository.java (JPA repository)
# - Product.java (you still need to create this manually though)

# Create a utility class
neojava make class StringUtils --package com.example.utils

# Create a main application class
neojava make class DemoApplication --main --package com.example
```

### Run Java Files

```bash
# Compile and run a Java file
neojava run Main.java

# You can omit the .java extension
neojava run Main

# Pass arguments to your program
neojava run MyApp --arg1 value1 --arg2

# What happens:
# 1. Compiles the Java file with javac
# 2. If compilation succeeds, runs the class with java
# 3. Passes any additional arguments to your program
```

## What's Next?

I might add:

- More templates (DTOs, mappers, exceptions)
- Configuration file for custom templates
- Support for Gradle projects
- Option to add common dependencies

## The "Aha!" Moment

The first time I pressed `<leader>jc` in Neovim and a fully-formed Java class appeared with my cursor inside, ready to write code... that felt magical.

This is exactly why I love programming - identifying friction in my workflow, building a solution, and making my tools work for ME instead of the other way around.

## Contributing

Got ideas? Found a bug? Want to add templates for your favorite framework? PRs welcome!

This tool solved MY problem. Maybe it'll solve yours too. Or at least inspire you to build your own solution.

## License

MIT - Do whatever you want with it.

---

_Manually creating Java files is for suckers._

```

```
