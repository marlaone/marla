package fields

type RequiredString string

func RequiredStringFromString(s string) (RequiredString, error) {
	if s == "" {
		return RequiredString(""), &ErrRequiredStringEmpty{}
	}
	return RequiredString(s), nil
}

func MustRequiredStringFromString(s string) RequiredString {
	if s, err := RequiredStringFromString(s); err == nil {
		return s
	} else {
		panic(err)
	}
}

type ErrRequiredStringEmpty struct{}

func (e ErrRequiredStringEmpty) Error() string {
	return "required string is empty"
}
