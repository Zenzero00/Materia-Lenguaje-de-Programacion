package main

type Status string

const (
	Todo       Status = "Por hacer"
	InProgress Status = "En progreso"
	Done       Status = "Terminada"
)

type Task struct {
	ID          int    `json:"id"`
	Description string `json:"description"`
	Status      Status `json:"status"`
}

type TaskManager interface {
	Add(description string) error
	UpdateStatus(id int, status Status) error
	Delete(id int) error
	ListAll() ([]Task, error)
	ListByStatus(status Status) ([]Task, error)
}
