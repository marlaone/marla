package fields

// RequiredString represents a string that is required.
// The string must not be empty.
type RequiredString string

// RequiredStringFromString returns a RequiredString from a string.
// If the string is empty, it returns an error.
func RequiredStringFromString(s string) (RequiredString, error) {
	if s == "" {
		return RequiredString(""), &ErrRequiredStringEmpty{}
	}
	return RequiredString(s), nil
}

// MustRequiredStringFromString returns a RequiredString from a string.
// If the string is empty, it panics.
func MustRequiredStringFromString(s string) RequiredString {
	if s, err := RequiredStringFromString(s); err == nil {
		return s
	} else {
		panic(err)
	}
}

// ErrRequiredStringEmpty is returned if the string is empty.
type ErrRequiredStringEmpty struct{}

// Error returns the error message.
func (e ErrRequiredStringEmpty) Error() string {
	return "required string is empty"
}
