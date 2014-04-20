.PHONY: run

calculator: main.rs
	rustc -o $@ $<

run: calculator
	@./calculator
