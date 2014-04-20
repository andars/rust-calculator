.PHONY: run

run: calculator
	@./calculator

calculator: main.rs
	rustc -o $@ $<
