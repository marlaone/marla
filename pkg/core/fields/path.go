package fields

import (
	"fmt"
	"os"
	"path/filepath"
)

// Path represents a path.
type Path string

// MustPathFromString returns a Path from a string.
// If the path is invalid, it panics.
func MustPathFromString(s string) Path {
	if path, err := PathFromString(s); err == nil {
		return path
	} else {
		panic(err)
	}
}

// PathFromString returns a Path from a string.
// If the path is invalid, it returns an error.
func PathFromString(s string) (Path, error) {
	if _, err := os.Stat(s); err == nil {
		return Path(s), nil
	} else {
		return Path(""), &ErrInvalidPath{pathError: err.(*os.PathError)}
	}
}

// String returns the path as a string.
func (p Path) String() string {
	return string(p)
}

// Join joins the path with the given elements.
func (p Path) Join(elem ...string) (Path, error) {
	paths := []string{p.String()}
	paths = append(paths, elem...)
	if path, err := PathFromString(filepath.Join(paths...)); err == nil {
		return path, nil
	} else {
		return Path(""), err
	}
}

// File opens the file and returns a file handle if the path is a file.
func (p Path) File() (*os.File, error) {
	if file, err := os.Open(p.String()); err == nil {
		return file, nil
	} else {
		return nil, err
	}
}

// ErrInvalidPath is returned if the path is invalid.
type ErrInvalidPath struct {
	pathError *os.PathError
}

// Error returns the error message.
func (e ErrInvalidPath) Error() string {
	return fmt.Sprintf("invalid path: %v", e.pathError)
}
