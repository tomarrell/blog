package main

import (
	"embed"
	_ "embed"
	"fmt"
	"html/template"
	"io/fs"
	"net/http"
	"time"

	"github.com/gomarkdown/markdown"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/rs/zerolog/log"
)

var (
	//go:embed posts/*
	//go:embed public/*
	//go:embed templates/*
	templateFS embed.FS
)

func main() {
	templates, err := template.ParseFS(templateFS, "templates/*")
	if err != nil {
		log.Fatal().Err(err).Msg("parsing templates")
	}

	public, err := fs.Sub(templateFS, "public")
	if err != nil {
		log.Fatal().Msg("failed to open subdir /public in embedded fs")
	}
	pubFS := http.FS(public)

	e := echo.New()
	e.HideBanner = true
	e.Use(middleware.LoggerWithConfig(middleware.LoggerConfig{
		Skipper: middleware.DefaultSkipper,
		Format:  format,
	}))
	e.HTTPErrorHandler = handleError(templates)

	e.GET("/", handleIndex(templates))
	e.GET("/post/:post", handlePost(templates))
	e.GET("/public/*", echo.WrapHandler(http.StripPrefix("/public", http.FileServer(pubFS))))
	e.GET("/favicon.ico", echo.WrapHandler(http.FileServer(pubFS)))

	log.Info().Msg("starting server")

	if err := e.Start(":8080"); err != nil {
		log.Err(err).Msg("server stopped")
	}
}

type postData struct {
	Title   string
	Date    time.Time
	Content template.HTML
}

func handlePost(t *template.Template) echo.HandlerFunc {
	return func(c echo.Context) error {
		postName := c.Param("post")
		log.Debug().Str("post", postName).Msg("fetching post")

		p, err := parsePost(postName)
		if err != nil {
			return fmt.Errorf("parsing post: %v", err)
		}

		html := markdown.ToHTML([]byte(p.Content), nil, nil)

		d := postData{
			Title:   p.Title,
			Date:    p.Date,
			Content: template.HTML(html),
		}

		if err := t.ExecuteTemplate(c.Response().Writer, "post.html.tmpl", d); err != nil {
			return err
		}

		return nil
	}
}

type indexData struct {
	Posts []Post
}

func handleIndex(t *template.Template) echo.HandlerFunc {
	return func(c echo.Context) error {
		posts, err := parsePosts()
		if err != nil {
			return echo.NewHTTPError(http.StatusInternalServerError, err)
		}

		d := indexData{
			Posts: posts,
		}

		if err := t.ExecuteTemplate(c.Response().Writer, "index.html.tmpl", d); err != nil {
			return echo.NewHTTPError(http.StatusInternalServerError, err)
		}

		return nil
	}
}

func handleError(t *template.Template) func(error, echo.Context) {
	return func(err error, c echo.Context) {
		code := http.StatusInternalServerError
		if he, ok := err.(*echo.HTTPError); ok {
			code = he.Code
		}

		var writeErr error

		switch code {
		case http.StatusNotFound:
			c.Response().WriteHeader(code)
			writeErr = t.ExecuteTemplate(c.Response().Writer, "404.html.tmpl", nil)
		default:
			c.Logger().Error(err)
			c.String(code, err.Error())
		}

		if writeErr != nil {
			log.Err(writeErr).Msg("writing error")
		}
	}
}

func loggingHandler(next http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		log.Debug().
			Str("method", r.Method).
			Str("host", r.Host).
			Str("url", r.URL.String()).
			Str("remoteAddr", r.RemoteAddr).
			Str("agent", r.UserAgent()).
			Str("referrer", r.Referer()).
			Msg("")

		next(w, r)
	}
}

func respondError(w http.ResponseWriter, err error) {
	log.Err(err).Msg("internal server error")
	http.Error(w, err.Error(), http.StatusInternalServerError)
}
