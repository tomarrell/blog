package main

import "io/fs"

var (
	//go:embed templates/*
	templateFS fs.FS
)

func main() {

}
