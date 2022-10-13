# Tom's Blog

Source for my personal blog.

A live version is hosted at [blog.tomarrell.com](https://blog.tomarrell.com).

## Setup

To run the project, clone into a directory and run.

```bash
$ go run .
```

The server will begin listening on port `8080`.

## Writing content

The blog works by parsing **Post** files, which are a custom Toml format
including all the fields needed to render the page. The posts are contained in
the `./posts` directory.

The files in this directory will be parsed, and rendered before being returned
to the client.

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

This is some content for the blog post. It supports the full CommonMark spec by
default. However can be expanded to the Github Markdown spec.

---

**Text**

**_Formatting_**

* Hello
* World

![](/public/images/turtle.jpg)
"""
```

The file name of each post will be designated as its path when serving.
Therefore, once a post is published, avoid changing the file name to prevent
accidentally changing the served URL path.

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
