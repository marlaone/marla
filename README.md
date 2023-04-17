# Marla Website

Project Marla is a tool to help you build websites. It is a static site generator that uses Markdown files as input and generates a static website as output.

## Features

- Markdown files as input
- Static website as output
- Customizable templates
- Customizable CSS
- Customizable JavaScript
- Customizable HTML

## Usage

1. Install

```sh
go install github.com/marlaone/marla/cmd
```

2. Start Website

Navigate to the root of your website and run the following command:

```sh
marla
```

This will start a web server on port 1818. You can now view your website by navigating to http://localhost:1818.

The application searches for a `config.yml` file in the root of your website, inside a `./site` directory, or inside a `~/.marla/` directory. If it finds a `config.yml` file, it will use that file. If it does not find a `config.yml` file, it will use the default configuration.

## Configuration

```yaml
content_path: "./site/content" # path to your markdown files
theme_path: "./site/themes/your_theme" # path to your theme
http_host: "localhost" # host to listen on
http_port: 1818 # port to listen on
default_language: "en" # default language for your website
```

## Website Directory Structure

```sh
. # Root
├── site # Site Directory
│   ├── config.yml # Configuration File
│   ├── content # Content Directory
│   │   ├── index.md # Home Page
│   │   ├── about.md # About Page
│   ├── theme # Theme Directory
│   │   ├── your_theme # Your Theme Directory
│   │   │   ├── templates # Templates Directory
│   │   │   │   ├── layout.html # Base Template
│   │   │   │   ├── index.html # Home Page Template
│   │   │   │   ├── page.html # About Page Template
|   |   |   |   ├── 404.html # 404 Page Template
│   │   │   ├── static # Static Directory
```

## Page Metadata

```yaml
title: "Home" # Page Title, required or a level 1 heading is required
description: "This is the home page." # Page Description
date: "2020-01-01" # Created Date
last_modified: "2020-01-01" # Last Modified Date
draft: true # Is Draft, default: false
slug: "home" # Page Slug, default: "{filename}"
template: "index.html" # Template File, default: "page.html"
path: "/home" # Page Path, default: "/{slug}"
aliases: ["/home", "/home.html"] # Page Aliases
authors: ["John Doe"] # Page Authors
tags: ["home", "index"] # Page Tags
extra: # Extra Metadata
  foo: "bar"
```
