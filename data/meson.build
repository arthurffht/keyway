# SPDX-FileCopyrightText: Keyway contributors
# SPDX-License-Identifier: CC0-1.0

conf = configuration_data()
conf.set('APP_ID', app_id)
conf.set('OBJECT_PATH', object_path)
conf.set('libexecdir', libexecdir)

# The desktop entry and metainfo describe the preferences app, so they are installed with it
# in Phase 5. The search provider waits for them too: GNOME Shell ignores a provider whose
# DesktopId does not resolve to a visible app (ui/remoteSearch.js).

desktop_in = configure_file(
  input: base_id + '.desktop.in',
  output: app_id + '.desktop.in',
  configuration: conf,
)
desktop_file = i18n.merge_file(
  input: desktop_in,
  output: app_id + '.desktop',
  type: 'desktop',
  po_dir: '../po',
  build_by_default: true,
)

metainfo_in = configure_file(
  input: base_id + '.metainfo.xml.in',
  output: app_id + '.metainfo.xml.in',
  configuration: conf,
)
metainfo_file = i18n.merge_file(
  input: metainfo_in,
  output: app_id + '.metainfo.xml',
  po_dir: '../po',
  build_by_default: true,
)

configure_file(
  input: 'dbus' / base_id + '.Engine.service.in',
  output: app_id + '.Engine.service',
  configuration: conf,
  install: true,
  install_dir: datadir / 'dbus-1' / 'services',
)

configure_file(
  input: 'search-provider' / base_id + '.search-provider.ini.in',
  output: app_id + '.search-provider.ini',
  configuration: conf,
)

install_data(base_id + '.gschema.xml',
  install_dir: datadir / 'glib-2.0' / 'schemas',
)

desktop_file_validate = find_program('desktop-file-validate', required: false)
if desktop_file_validate.found()
  test('validate-desktop', desktop_file_validate,
    args: [desktop_file],
    suite: 'data',
  )
endif

appstreamcli = find_program('appstreamcli', required: false)
if appstreamcli.found()
  test('validate-metainfo', appstreamcli,
    args: ['validate', '--no-net', '--explain', metainfo_file],
    suite: 'data',
  )
endif

glib_compile_schemas = find_program('glib-compile-schemas', required: false)
if glib_compile_schemas.found()
  test('validate-schema', glib_compile_schemas,
    args: ['--strict', '--dry-run', meson.current_source_dir()],
    suite: 'data',
  )
endif
