package ports

type LoggerPort interface {
	// Debugf logs a debug message with optional fields.
	Debug(msg string, keysAndValues ...any)
	// Infof logs an info message with optional fields.
	Info(msg string, keysAndValues ...any)
	// Warnf logs a warning message with optional fields.
	Warn(msg string, keysAndValues ...any)
	// Errorf logs an error message with optional fields.
	Error(msg string, keysAndValues ...any)
	// Fatalf logs a fatal message with optional fields.
	Fatal(msg string, keysAndValues ...any)
	// Panicf logs a panic message with optional fields.
	Panic(msg string, keysAndValues ...any)
}
