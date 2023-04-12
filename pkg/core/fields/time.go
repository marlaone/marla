package fields

import (
	"fmt"
	"time"
)

func TimeFromString(s string) (time.Time, error) {
	t, err := time.Parse(time.RFC3339, s)
	if err != nil {
		return time.Time{}, fmt.Errorf("invalid time: %v", err)
	}
	return t, nil
}
