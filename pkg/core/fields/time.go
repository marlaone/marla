package fields

import (
	"fmt"
	"time"
)

// TimeFromString returns a time.Time from a string.
// If the string is not a valid time, it returns an error.
// The string must be in RFC3339 format.
func TimeFromString(s string) (time.Time, error) {
	t, err := time.Parse(time.RFC3339, s)
	if err != nil {
		return time.Time{}, fmt.Errorf("invalid time: %v", err)
	}
	return t, nil
}
