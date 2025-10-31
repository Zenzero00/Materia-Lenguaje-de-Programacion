package main

import "time"

type Status string

const (
	Todo       Status = "Por hacer"
	InProgress Status = "En progreso"
	Done       Status = "Terminada"
)

type Task struct {
	ID          int       `json:"id"`
	Description string    `json:"description"`
	Status      Status    `json:"status"`
	CreatedAt   time.Time `json:"created_at"`
	UpdatedAt   time.Time `json:"updated_at"`
	CompletedAt time.Time `json:"completed_at,omitempty"`
}

type TaskManager interface {
	Add(description string) error
	UpdateStatus(id int, status Status) error
	Delete(id int) error
	ListAll() ([]Task, error)
	ListByStatus(status Status) ([]Task, error)
}
