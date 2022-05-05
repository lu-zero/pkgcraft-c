atom = executable('atom', f'atom.c', dependencies : [pkgcraft])
test('nonversion', atom, args : ['cat/pkg'], suite: 'atom',
  env : ['category=cat', 'package=pkg'])
test('version', atom, args : ['=cat/pkg-1'], suite: 'atom',
  env : ['category=cat', 'package=pkg', 'version=1'])
test('slotted', atom, args : ['cat/pkg:1'], suite: 'atom',
  env : ['category=cat', 'package=pkg', 'slot=1'])
test('subslot', atom, args : ['cat/pkg:1/2'], suite: 'atom',
  env : ['category=cat', 'package=pkg', 'slot=1', 'subslot=2'])
test('repo', atom, args : ['cat/pkg::repo'], suite: 'atom',
  env : ['category=cat', 'package=pkg', 'repo=repo'])

atom_cmp = executable('atom_cmp', f'atom_cmp.c', dependencies : [pkgcraft])
test('equal', atom_cmp, args : ['cat/pkg', 'cat/pkg', '0'])
test('less', atom_cmp, args : ['cat/apkg', 'cat/bpkg', '-1'])
test('greater', atom_cmp, args : ['=cat/pkg-2', '=cat/pkg-1', '1'])
