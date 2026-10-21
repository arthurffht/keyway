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
datadir = prefix / get_option('datadir')
libexecdir = prefix / get_option('libexecdir')
gettext_package = meson.project_name()

subdir('data')
subdir('po')

gnome.post_install(
  glib_compile_schemas: true,
  gtk_update_icon_cache: true,
)
