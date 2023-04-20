package utils

import (
	"net/url"
	"regexp"
	"strings"
)

var languageFileRegex = regexp.MustCompile(`\.(?P<Lang>[A-Za-z+-]+)$`)

func GetPageURL(baseURL string, pagePath string) (*url.URL, string) {
	uriPath := pagePath
	if !strings.HasPrefix(uriPath, "/") {
		uriPath = "/" + uriPath
	}
	uriPath = strings.TrimSuffix(uriPath, ".md")
	uriPath = strings.TrimSuffix(uriPath, "/index")

	if uriPath == "" {
		uriPath = "/"
	}

	if baseURL != "" {
		if !strings.HasSuffix(baseURL, "/") {
			baseURL += "/"
		}
		uriPath = strings.TrimPrefix(uriPath, "/")
		uriPath = baseURL + uriPath
	}

	langMatches := languageFileRegex.FindStringSubmatch(uriPath)

	language := ""
	if len(langMatches) > 1 {
		language = langMatches[1]
		uriPath = strings.TrimSuffix(uriPath, "."+langMatches[1])
	}

	return &url.URL{Path: uriPath, OmitHost: true}, language
}
