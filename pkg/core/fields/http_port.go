package fields

import (
	"fmt"
	"strconv"
)

type HttpPort int

func MustHttpPortFromInt(port int) HttpPort {
	if port, err := HttpPortFromInt(port); err == nil {
		return port
	} else {
		panic(err)
	}
}

func HttpPortFromInt(port int) (HttpPort, error) {
	if port < 0 || port > 65535 {
		return HttpPort(0), &ErrInvalidHttpPort{port: strconv.Itoa(port), reason: "port number must be between 0 and 65535"}
	}
	return HttpPort(port), nil
}

func MustHttpPortFromString(port string) HttpPort {
	if port, err := HttpPortFromString(port); err == nil {
		return port
	} else {
		panic(err)
	}
}

func HttpPortFromString(port string) (HttpPort, error) {
	portNum, err := strconv.Atoi(port)
	if err != nil {
		return HttpPort(0), &ErrInvalidHttpPort{port: port, reason: err.Error()}
	}
	return HttpPortFromInt(portNum)
}

type ErrInvalidHttpPort struct {
	port   string
	reason string
}

func (e ErrInvalidHttpPort) Error() string {
	return fmt.Sprintf("invalid http port %s: %s", e.port, e.reason)
}
