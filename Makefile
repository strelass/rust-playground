compile:
	rustc src/${d}/${f}.rs

run:
	./${f}

cleanup:
	rm ./${f}

compile_and_run: compile run cleanup
