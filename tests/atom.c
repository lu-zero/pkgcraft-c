#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <pkgcraft.h>

int main (int argc, char **argv) {
	char *atom, *expected;
	const char *value;
	Atom *a;

	if (argc == 2) {
		atom = argv[1];
	} else if (argc < 2) {
		fprintf(stderr, "missing required atom arg\n");
		exit(1);
	}

	a = pkgcraft_atom(atom, NULL);

	value = pkgcraft_atom_key(a);
	assert(strcmp(value, "cat/pkg") == 0);
	value = pkgcraft_atom_category(a);
	assert(strcmp(value, getenv("category")) == 0);
	value = pkgcraft_atom_package(a);
	assert(strcmp(value, getenv("package")) == 0);

	value = pkgcraft_atom_version(a);
	expected = getenv("version");
	if (expected != NULL) {
		assert(strcmp(value, expected) == 0);
	} else {
		assert(strcmp(value, "") == 0);
	}

	value = pkgcraft_atom_slot(a);
	expected = getenv("slot");
	if (expected != NULL) {
		assert(strcmp(value, expected) == 0);
	} else {
		assert(strcmp(value, "") == 0);
	}

	value = pkgcraft_atom_subslot(a);
	expected = getenv("subslot");
	if (expected != NULL) {
		assert(strcmp(value, expected) == 0);
	} else {
		assert(strcmp(value, "") == 0);
	}

	value = pkgcraft_atom_repo(a);
	expected = getenv("repo");
	if (expected != NULL) {
		assert(strcmp(value, expected) == 0);
	} else {
		assert(strcmp(value, "") == 0);
	}

	pkgcraft_atom_free(a);
	pkgcraft_free_str((char *)value);

	return 0;
}
