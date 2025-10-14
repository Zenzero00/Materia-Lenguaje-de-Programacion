package main

import (
	"encoding/json"
	"os"
)

type Storage interface {
	Load() ([]Task, error)
	Save(tasks []Task) error
}

type JSONStorage struct {
	filename string
}

func NewJSONStorage(filename string) *JSONStorage {
	return &JSONStorage{filename: filename}
}

func (s *JSONStorage) Load() ([]Task, error) {
	data, err := os.ReadFile(s.filename)
	if err != nil {
		if os.IsNotExist(err) {
			return []Task{}, nil
		}
		return nil, err
	}

	var tasks []Task
	if err := json.Unmarshal(data, &tasks); err != nil {
		return nil, err
	}
	return tasks, nil
}

func (s *JSONStorage) Save(tasks []Task) error {
	data, err := json.MarshalIndent(tasks, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(s.filename, data, 0644)
}
