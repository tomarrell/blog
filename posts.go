package main

import (
	"fmt"
	"html/template"
	"io/fs"
	"path"
	"path/filepath"
	"sort"
	"strings"
	"time"

	"github.com/pelletier/go-toml"
	"github.com/rs/zerolog/log"
)

const (
	dateFormat = "2006-01-02T15:04:05-0700"

	postsPath     = "posts"
	tomlExtension = ".toml"
)

type Post struct {
	ID      string    `toml:"_"` // File name
	Title   string    `toml:"title"`
	RawDate string    `toml:"date"`
	Date    time.Time `toml:"_"`
	Og      struct {
		Description string `toml:"description"`
		Image       string `toml:"image"`
	} `toml:"og"`
	RawDescription string        `toml:"description"`
	Description    template.HTML `toml:"_"`
	Content        string        `toml:"content"`
}

func parsePosts() ([]Post, error) {
	var posts []Post

	entries, err := fs.ReadDir(templateFS, postsPath)
	if err != nil {
		return nil, fmt.Errorf("reading posts directory: %v", err)
	}

	for _, e := range entries {
		if e.IsDir() {
			log.Warn().Str("name", e.Name()).Msg("entry is a directory")
			continue
		}

		p := path.Join(postsPath, e.Name())
		if !strings.Contains(p, tomlExtension) {
			continue
		}

		b, err := fs.ReadFile(templateFS, p)
		if err != nil {
			log.Err(err).Str("name", e.Name()).Msg("reading file")
			continue
		}

		var post Post
		if err := toml.Unmarshal(b, &post); err != nil {
			log.Err(err).Str("name", e.Name()).Msg("unmarshallling post")
			continue
		}

		postDate, err := time.Parse(dateFormat, post.RawDate)
		if err != nil {
			log.Err(err).Str("name", e.Name()).Msg("parsing date")
			continue
		}

		post.Date = postDate
		post.ID = strings.TrimSuffix(e.Name(), filepath.Ext(e.Name()))
		post.Description = template.HTML(post.RawDescription)

		posts = append(posts, post)
	}

	sort.Slice(posts, func(i, j int) bool {
		return posts[i].Date.After(posts[j].Date)
	})

	return posts, nil
}

func parsePost(name string) (Post, error) {
	p := path.Join(postsPath, name) + tomlExtension

	b, err := fs.ReadFile(templateFS, p)
	if err != nil {
		return Post{}, fmt.Errorf("reading post %s: %v", p, err)
	}

	var post Post
	if err := toml.Unmarshal(b, &post); err != nil {
		return Post{}, fmt.Errorf("unmarshalling post %s: %v", p, err)
	}

	postDate, err := time.Parse(dateFormat, post.RawDate)
	if err != nil {
		return Post{}, fmt.Errorf("parsing post date %s: %v", p, err)
	}

	post.Date = postDate
	post.ID = strings.TrimSuffix(name, filepath.Ext(name))

	return post, nil
}
