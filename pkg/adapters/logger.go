package adapters

import (
	"fmt"

	"github.com/marlaone/marla/pkg/core/ports"
	"github.com/spf13/viper"
	"go.uber.org/zap"
)

type LoggerAdapter struct {
	logger *zap.Logger
}

var _ ports.LoggerPort = &LoggerAdapter{}

func NewLoggerAdapter() *LoggerAdapter {
	var logger *zap.Logger
	var err error
	if viper.GetString("enviroment") == "production" {
		logger, err = zap.NewProduction()
		if err != nil {
			panic(err)
		}
	} else {
		logger, err = zap.NewDevelopment()
		if err != nil {
			panic(err)
		}
	}
	return &LoggerAdapter{logger: logger}
}

func (l *LoggerAdapter) getFields(keysAndValues ...interface{}) []zap.Field {
	var fields []zap.Field
	for i := 0; i < len(keysAndValues); i += 2 {
		fields = append(fields, zap.Any(fmt.Sprintf("%v", keysAndValues[i]), keysAndValues[i+1]))
	}
	return fields
}

// Debug logs a debug message with optional fields.
func (l *LoggerAdapter) Debug(msg string, keysAndValues ...any) {
	fields := l.getFields(keysAndValues...)
	l.logger.Debug(msg, fields...)
}

// Info logs an info message with optional fields.
func (l *LoggerAdapter) Info(msg string, keysAndValues ...any) {
	fields := l.getFields(keysAndValues...)
	l.logger.Info(msg, fields...)
}

// Warn logs a warning message with optional fields.
func (l *LoggerAdapter) Warn(msg string, keysAndValues ...any) {
	fields := l.getFields(keysAndValues...)
	l.logger.Warn(msg, fields...)
}

// Error logs an error message with optional fields.
func (l *LoggerAdapter) Error(msg string, keysAndValues ...any) {
	fields := l.getFields(keysAndValues...)
	l.logger.Error(msg, fields...)
}

// Fatal logs a fatal message with optional fields.
func (l *LoggerAdapter) Fatal(msg string, keysAndValues ...any) {
	fields := l.getFields(keysAndValues...)
	l.logger.Fatal(msg, fields...)
}

// Panic logs a panic message with optional fields.
func (l *LoggerAdapter) Panic(msg string, keysAndValues ...any) {
	fields := l.getFields(keysAndValues...)
	l.logger.Panic(msg, fields...)
}
