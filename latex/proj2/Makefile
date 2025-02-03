all: pdf

pdf:
	latexmk -pdf -shell-escape -auxdir=aux/ main.tex

clean:
	rm -rf aux/
	latexmk -c

.PHONY: all pdf clean
