#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <pkgcraft.h>

int main (int argc, char **argv) {
	char *atom, *expected;
	char *value;
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
	pkgcraft_str_free(value);
	value = pkgcraft_atom_category(a);
	assert(strcmp(value, getenv("category")) == 0);
	pkgcraft_str_free(value);
	value = pkgcraft_atom_package(a);
	assert(strcmp(value, getenv("package")) == 0);
	pkgcraft_str_free(value);

	value = pkgcraft_atom_version(a);
	expected = getenv("version");
	if (expected) {
		assert(strcmp(value, expected) == 0);
		pkgcraft_str_free(value);
	} else {
		assert(value == expected);
	}

	value = pkgcraft_atom_slot(a);
	expected = getenv("slot");
	if (expected) {
		assert(strcmp(value, expected) == 0);
		pkgcraft_str_free(value);
	} else {
		assert(value == expected);
	}

	value = pkgcraft_atom_subslot(a);
	expected = getenv("subslot");
	if (expected) {
		assert(strcmp(value, expected) == 0);
		pkgcraft_str_free(value);
	} else {
		assert(value == expected);
	}

	value = pkgcraft_atom_repo(a);
	expected = getenv("repo");
	if (expected) {
		assert(strcmp(value, expected) == 0);
		pkgcraft_str_free(value);
	} else {
		assert(value == expected);
	}

	pkgcraft_atom_free(a);

	return 0;
}
