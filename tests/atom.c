#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <pkgcraft.h>

int main (int argc, char **argv) {
	char *atom, *val;
	Atom *a;

	if (argc == 2) {
		atom = argv[1];
	} else if (argc < 2) {
		fprintf(stderr, "missing required atom arg\n");
		exit(1);
	}

	a = pkgcraft_atom(atom, NULL);

	assert(strcmp(a->category, getenv("category")) == 0);
	assert(strcmp(a->package, getenv("package")) == 0);

	val = getenv("version");
	if (val != NULL) {
		assert(strcmp(a->version, val) == 0);
	} else {
		assert(a->version == NULL);
	}

	val = getenv("slot");
	if (val != NULL) {
		assert(strcmp(a->slot, val) == 0);
	} else {
		assert(a->slot == NULL);
	}

	val = getenv("subslot");
	if (val != NULL) {
		assert(strcmp(a->subslot, val) == 0);
	} else {
		assert(a->subslot == NULL);
	}

	val = getenv("repo");
	if (val != NULL) {
		assert(strcmp(a->repo, val) == 0);
	} else {
		assert(a->repo == NULL);
	}

	pkgcraft_atom_free(a);

	return 0;
}
