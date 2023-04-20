package fields

import (
	"fmt"
	"strconv"
)

// HttpPort represents a port number.
// The port number must be between 0 and 65535.
type HttpPort int

// MustHttpPortFromInt returns a HttpPort from an int.
// If the port number is invalid, it panics.
func MustHttpPortFromInt(port int) HttpPort {
	if port, err := HttpPortFromInt(port); err == nil {
		return port
	} else {
		panic(err)
	}
}

// HttpPortFromInt returns a HttpPort from an int.
// If the port number is invalid, it returns an error.
// The port number must be between 0 and 65535.
func HttpPortFromInt(port int) (HttpPort, error) {
	if port < 0 || port > 65535 {
		return HttpPort(0), &ErrInvalidHttpPort{port: strconv.Itoa(port), reason: "port number must be between 0 and 65535"}
	}
	return HttpPort(port), nil
}

// MustHttpPortFromString returns a HttpPort from a string.
// If the port number is invalid, it panics.
func MustHttpPortFromString(port string) HttpPort {
	if port, err := HttpPortFromString(port); err == nil {
		return port
	} else {
		panic(err)
	}
}

// HttpPortFromString returns a HttpPort from a string.
// If the port number is invalid, it returns an error.
// The port number must be between 0 and 65535.
func HttpPortFromString(port string) (HttpPort, error) {
	portNum, err := strconv.Atoi(port)
	if err != nil {
		return HttpPort(0), &ErrInvalidHttpPort{port: port, reason: err.Error()}
	}
	return HttpPortFromInt(portNum)
}

// ErrInvalidHttpPort is returned if the port number is invalid.
type ErrInvalidHttpPort struct {
	port   string
	reason string
}

// Error returns the error message.
func (e ErrInvalidHttpPort) Error() string {
	return fmt.Sprintf("invalid http port %s: %s", e.port, e.reason)
}
