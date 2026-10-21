# SPDX-FileCopyrightText: Keyway contributors
# SPDX-License-Identifier: CC0-1.0

project('keyway',
  version: '0.1.0',
  license: 'GPL-3.0-or-later',
  meson_version: '>= 1.4',
)

i18n = import('i18n')
gnome = import('gnome')

base_id = 'io.github.arthurffht.Keyway'
if get_option('profile') == 'development'
  app_id = base_id + '.Devel'
else
  app_id = base_id
endif
object_path = '/io/github/arthurffht/Keyway'

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
datadir = prefix / get_option('datadir')
libexecdir = prefix / get_option('libexecdir')
gettext_package = meson.project_name()

cargo = find_program('cargo')
cargo_env = environment()
cargo_env.set('KEYWAY_APP_ID', app_id)

# Cargo tracks its own dependencies, so the target always runs and Cargo decides what to rebuild.
custom_target('cargo-build',
  build_by_default: true,
  build_always_stale: true,
  console: true,
  output: ['keyway-engine', 'keywayctl'],
  install: true,
  install_dir: [libexecdir, bindir],
  env: cargo_env,
  command: [
    find_program('build-aux/cargo.sh'),
    cargo,
    meson.project_source_root(),
    meson.project_build_root() / 'target',
    get_option('profile'),
    '@OUTPUT@',
  ],
)

subdir('data')
subdir('po')

gnome.post_install(
  glib_compile_schemas: true,
)
