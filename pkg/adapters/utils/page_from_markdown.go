package utils

import (
	"bytes"
	"fmt"
	"io"
	"net/url"
	"path/filepath"
	"regexp"
	"strings"
	"time"

	"github.com/marlaone/marla/pkg/core/entities"
	"github.com/marlaone/marla/pkg/core/fields"
	"github.com/yuin/goldmark"
	"github.com/yuin/goldmark/ast"
	"github.com/yuin/goldmark/extension"
	"github.com/yuin/goldmark/parser"
	"github.com/yuin/goldmark/renderer"
	"github.com/yuin/goldmark/renderer/html"
	"github.com/yuin/goldmark/text"
	"github.com/yuin/goldmark/util"
	"go.abhg.dev/goldmark/frontmatter"
)

var md = goldmark.New(
	goldmark.WithExtensions(
		extension.Linkify,
		&frontmatter.Extender{},
	),
)

var mdRenderer = renderer.NewRenderer(
	renderer.WithNodeRenderers(
		util.Prioritized(
			html.NewRenderer(
				html.WithUnsafe(),
			),
			1000,
		),
	),
)

var languageFileRegex = regexp.MustCompile(`\.(?P<Lang>[A-Za-z+-]+)$`)

type pageMeta struct {
	Title        string         `yaml:"title"`
	Description  string         `yaml:"description"`
	Date         string         `yaml:"date"`
	LastModified string         `yaml:"last_modified"`
	IsDraft      bool           `yaml:"draft"`
	Slug         string         `yaml:"slug"`
	Template     string         `yaml:"template"`
	Aliases      string         `yaml:"aliases"`
	Authors      []string       `yaml:"authors"`
	Tags         []string       `yaml:"tags"`
	Extra        map[string]any `yaml:"extra"`
}

func PageFromMarkdownFile(config *entities.Config, path fields.Path) (*entities.Page, error) {
	p := &entities.Page{}
	p.ContentPath = path
	p.Language = config.DefaultLangauge

	uriPath, err := filepath.Rel(config.ContentPath.String(), path.String())
	if err != nil {
		return p, fmt.Errorf("could not get relative path: %w", err)
	}
	if !strings.HasPrefix(uriPath, "/") {
		uriPath = "/" + uriPath
	}
	uriPath = strings.TrimSuffix(uriPath, ".md")
	uriPath = strings.TrimSuffix(uriPath, "/index")

	if uriPath == "" {
		uriPath = "/"
	}

	langMatches := languageFileRegex.FindStringSubmatch(uriPath)

	if len(langMatches) > 1 {
		p.Language = fields.MustRequiredStringFromString(langMatches[1])
		uriPath = strings.TrimSuffix(uriPath, "."+langMatches[1])
	}

	p.Path = &url.URL{Path: uriPath, OmitHost: true}

	f, err := p.ContentPath.File()
	if err != nil {
		return p, fmt.Errorf("could not open file %s: %w", path, err)
	}
	defer f.Close()

	plainContent, err := io.ReadAll(f)
	if err != nil {
		return p, fmt.Errorf("could not read file %s: %w", path, err)
	}

	p.PlainContent = plainContent

	buf := bytes.NewBuffer(nil)

	ctx := parser.NewContext()
	reader := text.NewReader(p.PlainContent)
	doc := md.Parser().Parse(reader, parser.WithContext(ctx))

	if err := mdRenderer.Render(buf, p.PlainContent, doc); err != nil {
		return p, fmt.Errorf("could not convert markdown to html: %w", err)
	}

	p.Content = buf.Bytes()

	d := frontmatter.Get(ctx)

	meta := &pageMeta{}
	if d != nil {
		if err := d.Decode(&meta); err != nil {
			return p, fmt.Errorf("could not decode page meta: %w", err)
		}
	}

	if meta.Title == "" {
		meta.Title = extractH1Text(doc, p.PlainContent)
	}

	title, err := fields.RequiredStringFromString(meta.Title)
	if err != nil {
		return p, fmt.Errorf("missing title for page, either `title: \"example\"` or a level 1 heading are required: %w", err)
	}

	if meta.Slug != "" {
		if !strings.HasPrefix(meta.Slug, "/") {
			meta.Slug = "/" + meta.Slug
		}
		p.Path = &url.URL{Path: meta.Slug, OmitHost: true}
	}

	createdAt, err := fields.TimeFromString(meta.Date)
	if err != nil {
		p.CreatedAt = time.Now()
	}

	lastModifiedAt, err := fields.TimeFromString(meta.LastModified)
	if err != nil {
		p.LastModifiedAt = nil
	}

	templatePath, err := fields.PathFromString(meta.Template)
	if err != nil {
		if meta.Template != "" {
			// if the template is not empty, search for it in the theme
			templatePath, err = config.ThemePath.Join("templates", meta.Template)
			if err != nil {
				// otherwise, use the default page template
				templatePath, err = config.ThemePath.Join("templates", "page.html")
				if err != nil {
					return p, fmt.Errorf("could not create page template field: %w", err)
				}
			}
		} else {
			// if the template is invalid or empty, use the default page template
			templatePath, err = config.ThemePath.Join("templates", "page.html")
			if err != nil {
				return p, fmt.Errorf("could not create page template field: %w", err)
			}
		}
	}

	p.Title = title
	p.CreatedAt = createdAt
	p.LastModifiedAt = &lastModifiedAt
	p.IsDraft = meta.IsDraft
	p.Template = templatePath
	p.Authors = meta.Authors
	p.Tags = meta.Tags
	p.Extra = meta.Extra
	p.Words = uint(len(string(p.PlainContent)))

	return p, nil
}

type H1TextExtractor struct {
	Segment text.Segment
}

func (h *H1TextExtractor) Visit(node ast.Node, entering bool) ast.WalkStatus {
	if !entering {
		return ast.WalkContinue
	}
	if heading, ok := node.(*ast.Heading); ok && heading.Level == 1 {
		h.Segment = heading.Lines().At(0)
		return ast.WalkStop
	}
	return ast.WalkContinue
}

func extractH1Text(n ast.Node, contentPlain []byte) string {
	extractor := &H1TextExtractor{}
	ast.Walk(n, func(n ast.Node, entering bool) (ast.WalkStatus, error) {
		return extractor.Visit(n, entering), nil
	})

	content := string(contentPlain)
	return content[extractor.Segment.Start:extractor.Segment.Stop]
}
