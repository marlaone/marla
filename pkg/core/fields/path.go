package fields

import (
	"fmt"
	"os"
	"path/filepath"
)

type Path string

func MustPathFromString(s string) Path {
	if path, err := PathFromString(s); err == nil {
		return path
	} else {
		panic(err)
	}
}

func PathFromString(s string) (Path, error) {
	if _, err := os.Stat(s); err == nil {
		return Path(s), nil
	} else {
		return Path(""), &ErrInvalidPath{pathError: err.(*os.PathError)}
	}
}

func (p Path) String() string {
	return string(p)
}

func (p Path) Join(elem ...string) (Path, error) {
	paths := []string{p.String()}
	paths = append(paths, elem...)
	if path, err := PathFromString(filepath.Join(paths...)); err == nil {
		return path, nil
	} else {
		return Path(""), err
	}
}

func (p Path) File() (*os.File, error) {
	if file, err := os.Open(p.String()); err == nil {
		return file, nil
	} else {
		return nil, err
	}
}

type ErrInvalidPath struct {
	pathError *os.PathError
}

func (e ErrInvalidPath) Error() string {
	return fmt.Sprintf("invalid path: %v", e.pathError)
}
