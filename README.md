## Tom's Blog
Source for my personal blog, written in Rust.

A live version is hosted at [blog.tomarrell.com](https://blog.tomarrell.com).

Web server using [actix_web](https://github.com/actix/actix-web), toml parsing using [serde](https://github.com/serde-rs/serde), logging using [pretty_env_logger](https://github.com/seanmonstar/pretty-env-logger), html templating using [handlebars-rust](https://github.com/sunng87/handlebars-rust).

## Installation
If you would like to setup a local copy for yourself:
```bash
>> # clone the repository
>> cd blog
>> make start
```

This will compile the crate in *release* mode, and begin the server listening on port `8080`.

This command is also setup to specifically write output to `log.txt` in the project root. Feel free to change this behaviour to what suits.

## Writing content
The blog works by parsing **Post** files, which are a custom Toml format including all the fields needed to render the page. The posts are contained in the `./posts` directory.

The files in this directory will be parsed, and rendered before being returned to the client.

An example **Post** is below:
```toml
# Info
title = "Test Post"
date = "2019-01-01T00:00:00Z"
description = "A post description"

# Content
content = """

# This is an h1
## This is a h2
### This is an h3

...etc

This is some content for the blog post. It supports the full CommonMark spec by default. However can be expanded to the Github Markdown spec.

---

**Text**

**_Formatting_**

* Hello
* World

![](/public/images/turtle.jpg)

"""
```

The file name of each post will be designated as its *unique ID*. It also determines its sorted position. Therefore I recommend you use a naming convention of `YYYY-MM-DD-*.toml` in order to keep things orderly in filesystems.

## License
Licensed under the GNU GPL v3.0 license. Please see the extended license terms here.

Simple breakdown:

| **Permissions** | **Condition**                | **Limitations** |
|-----------------|------------------------------|-----------------|
| Commercial use  | Disclose source              | Liability       |
| Distribution    | License and copyright notice | Warranty        |
| Modification    | Same license                 |
| Patent use      | State changes                |
| Private use     |
