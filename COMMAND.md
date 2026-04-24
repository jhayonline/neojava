## 📦 Regular Java Commands

### Classes

```bash
# Basic class
neojava make class User

# Class with main method
neojava make class Main --main

# Class with package
neojava make class UserService --package com.example.services

# Class with custom path
neojava make class Product --path src/main/java

# Class with package AND custom path
neojava make class OrderService --path backend --package com.example.services

# Class with everything
neojava make class Application --main --package com.example --path src
```

### Interfaces

```bash
# Basic interface
neojava make interface Repository

# Interface with package
neojava make interface CrudRepository --package com.example.data

# Interface with custom path
neojava make interface Authenticatable --path core
```

### Enums

```bash
# Basic enum
neojava make enum Status

# Enum with package
neojava make enum Role --package com.example.models

# Enum with path
neojava make enum Priority --path types
```

### Records

```bash
# Basic record
neojava make record Person

# Record with package
neojava make record Address --package com.example.valueobjects

# Record with path
neojava make record ProductDTO --path dto
```

### Abstract Classes

```bash
# Basic abstract class
neojava make abstract-class BaseService

# With package
neojava make abstract-class Repository --package com.example.base
```

### Sealed Classes

```bash
# Basic sealed class
neojava make sealed-class Shape

# With package
neojava make sealed-class PaymentMethod --package com.example.payments
```

## 🍃 Spring Boot Commands

### REST Controllers (Full CRUD)

```bash
# Basic REST controller (creates UserController.java)
neojava spring rest-controller User

# REST controller with custom package
neojava spring rest-controller Product --package com.myapp.controllers

# REST controller with custom path
neojava spring rest-controller Order --path backend

# With everything
neojava spring rest-controller Customer --path src/main/java --package com.shop.controllers
```

### MVC Controllers (View-based)

```bash
# Basic controller
neojava spring controller Home

# With package
neojava spring controller Admin --package com.dashboard.controllers
```

### Services

```bash
# Basic service (creates UserService.java)
neojava spring service User

# Service with custom package
neojava spring service ProductService --package com.myapp.services

# Service with custom path
neojava spring service EmailService --path core
```

### Repositories

```bash
# Basic repository (creates UserRepository.java)
neojava spring repository User

# Repository with package
neojava spring repository ProductRepository --package com.myapp.data

# Repository with custom path
neojava spring repository OrderRepository --path persistence
```

### Configurations

```bash
# Basic config class
neojava spring configuration Security

# Config with package
neojava spring configuration Database --package com.app.config

# Config with custom path
neojava spring configuration Swagger --path conf
```

### Components

```bash
# Basic component
neojava spring component EmailSender

# Component with package
neojava spring component PasswordEncoder --package com.app.security

# Component with custom path
neojava spring component CacheManager --path infrastructure
```

## 🎯 Combined Examples (Real World Usage)

### Building a User Management API

```bash
# Create everything for a User feature in one go
neojava spring rest-controller User
neojava spring service User
neojava spring repository User
neojava make class User --package com.example.models
neojava make enum Role --package com.example.models
neojava make record UserDTO --package com.example.dto
```

### Building an E-commerce System

```bash
# Product module
neojava spring rest-controller Product
neojava spring service Product
neojava spring repository Product
neojava make record ProductDTO --package com.shop.dto

# Order module
neojava spring rest-controller Order
neojava spring service Order
neojava spring repository Order
neojava make enum OrderStatus --package com.shop.models
```

### Building with Custom Structure

```bash
# Create with specific package structure
neojava make class User --package com.mycompany.api.v1.models
neojava spring rest-controller User --package com.mycompany.api.v1.controllers
neojava spring service User --package com.mycompany.api.v1.services
neojava spring repository User --package com.mycompany.api.v1.repositories

# Create in specific directories
neojava make class User --path backend/src/main/java --package com.myapp.models
neojava spring rest-controller Product --path api/src/main/java --package com.myapp.api
```

## 🔧 Help Commands

```bash
# General help
neojava --help

# List all templates
neojava list

# Version
neojava --version
```

## 🆕 Project Creation Commands

```bash
# Create a new Java project
neojava new MyApp

# Create with custom group ID
neojava new MyApp --group-id com.mycompany

# Create in specific directory
neojava new MyApp --path ~/projects

# What this creates:
neojava new MyApp
# Output:
# 🚀 Creating Java project: MyApp
#   ✓ Created Maven project structure
#   ✓ Generated pom.xml
#   ✓ Created .gitignore
#
# ✅ Project created successfully at: ./MyApp
#
# Next steps:
#   cd MyApp
#   neojava make class Main --main
#
# Build with:
#   mvn clean compile
#   mvn exec:java -Dexec.mainClass=com.example.Main
```

## Run Commands

```bash
# Compile and run a Java file
neojava run Main.java

# Omit the .java extension
neojava run Main

# Run with program arguments
neojava run Calculator 5 + 3
neojava run MyApp --verbose --config=settings.json

# What neojava run does:
# 1. Checks if the .java file exists
# 2. Runs `javac <file>.java`
# 3. If compilation succeeds, runs `java <class>`
# 4. Passes any extra arguments to your program

# Example with a simple program:
# File: Hello.java
# public class Hello {
#     public static void main(String[] args) {
#         System.out.println("Hello, " + args[0]);
#     }
# }

$ neojava run Hello.java World
# 🔨 Compiling Hello.java...
# ✅ Compilation successful!
# 🚀 Running Hello...
# ==================================================
# Hello, World!
```

````

## 💡 Pro Tips

### In Neovim (with keymaps configured)

```vim
" Create files right from your current buffer location
<leader>jc   " Class
<leader>ji   " Interface
<leader>je   " Enum
<leader>jr   " Record
<leader>jspc " Spring REST Controller
```

### File Naming Rules

- Names must start with UPPERCASE
- Only letters and numbers allowed
- Examples: `User`, `ProductService`, `OrderController`

### Generated Files Include

- File header with creation date
- Package declaration
- Proper imports
- Constructor
- Basic methods (CRUD for REST controllers)
- TODO comments to guide you

### What Each Spring Component Generates

**REST Controller** - Full CRUD with:

- GET /api/resource (list all)
- GET /api/resource/{id} (get one)
- POST /api/resource (create)
- PUT /api/resource/{id} (update)
- DELETE /api/resource/{id} (delete)

**Service** - Business logic layer with:

- findAll(), findById(), save(), update(), delete()

**Repository** - JPA repository interface
**Configuration** - Spring config with @Bean example
**Component** - Generic Spring component

## Quick Start Workflow

```bash
# 1. Create a new project
mkdir my-spring-app && cd my-spring-app

# 2. Generate all layers for a feature
neojava spring rest-controller Product
neojava spring service Product
neojava spring repository Product
neojava make class Product --package com.example.models

# 3. Open in Neovim and start coding
nvim ProductController.java
```
````
