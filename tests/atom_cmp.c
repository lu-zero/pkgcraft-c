#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#include <pkgcraft.h>

int main (int argc, char **argv) {
	char *s1, *s2;
	Atom *a1, *a2;
	int value, expected;

	if (argc == 4) {
		s1 = argv[1];
		s2 = argv[2];
		expected = atoi(argv[3]);
	} else if (argc < 4) {
		fprintf(stderr, "missing required atom args\n");
		exit(1);
	}

	a1 = pkgcraft_atom(s1, NULL);
	a2 = pkgcraft_atom(s2, NULL);
	value = pkgcraft_atom_cmp(a1, a2);
	assert(value == expected);

	return 0;
}
