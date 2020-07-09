Name:           neovim-gtk
Version:        {{{ git_version_describe }}}
Release:        1%{?dist}
Summary:        GTK ui for neovim.

License:        GPLv3+
URL:            https://github.com/daa84/neovim-gtk
VCS:            {{{ git_dir_vcs }}}

Source:         {{{ git_dir_pack }}}

BuildRequires:  atk-devel cairo-devel gdk-pixbuf2-devel glib2-devel gtk3-devel pango-devel cargo rust
Requires:       atk cairo gdk-pixbuf2 glib2 gtk3 pango

%description
GTK ui for neovim written in rust using gtk-rs bindings. With ligatures support.

%prep
{{{ git_dir_setup_macro }}}

%build
cargo build --release

%install
cargo install --path . --root %{buildroot}/usr

%files
/usr/bin/neovim-gtk

%changelog
{{{ git_dir_changelog }}}
