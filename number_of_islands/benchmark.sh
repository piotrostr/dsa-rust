#!/bin/bash

go test -bench=BenchmarkNumIslands -benchmem -cpuprofile cpu.out -memprofile mem.out

go tool pprof -http=:8080 cpu.out
