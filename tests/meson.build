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
