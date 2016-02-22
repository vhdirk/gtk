# gtk [![Build Status](https://travis-ci.org/gtk-rs/gtk.png?branch=master)](https://travis-ci.org/gtk-rs/gtk) [![Build status](https://ci.appveyor.com/api/projects/status/5mot32ipr12iocw0?svg=true)](https://ci.appveyor.com/project/GuillaumeGomez/gtk) [![Gitter](https://badges.gitter.im/Join Chat.svg)](https://gitter.im/gtk-rs/gtk)

[Project site](http://gtk-rs.org/) | [Online documentation](http://gtk-rs.org/docs/)

__Rust__ bindings and wrappers for __GLib__, __GDK 3__, __GTK+ 3__  and __Cairo__.

## Building

__gtk__ expects __GTK+__, __GLib__ and __Cairo__ development files to be installed on your system.
See the [requirements page](http://gtk-rs.org/docs/requirements.html).

## Using

We recommend using [crates from crates.io](https://crates.io/keywords/gtk-rs),
as [demonstrated here](http://gtk-rs.org/#using).

If you want to track the bleeding edge, use the git dependency instead:

```toml
[dependencies]
gtk = { git = "https://github.com/gtk-rs/gtk.git" }
```

Avoid mixing versioned and git crates like this:

```toml
# This will not compile
[dependencies]
gdk = "0.2"
gtk = { git = "https://github.com/gtk-rs/gtk.git" }
```

## Documentation

The majority of the documentation is kept [in a separate repo][gtk-md] due to
licensing issues. You can pull it in with cargo:

```shell
> cargo build --embed-lgpl-docs
```

Changes to those doc-comments should be submitted to the `lgpl-docs` repo. Avoid
including those embedded doc-comments in PRs to this repo.

The opposite feature removes all of those docs regardless of edits:

```shell
> cargo build --purge-lgpl-docs
```

These features **rewrite the crate sources** so it's sufficient to enable them
once. **Omitting them in the following `cargo` invocations will not undo their
effects!**

[gtk-md]: https://github.com/gtk-rs/lgpl-docs/blob/master/gtk.md

## Contribute

Contributor you're welcome!

See the general [bindings documentation](https://github.com/gtk-rs/glib/blob/master/src/lib.rs#L5).

Most of the bindings ([`src/auto`](src/auto)) are generated by [gir](https://github.com/gtk-rs/gir) using [this configuration file](Gir.toml). After editing `Gir.toml` the sources can be regenerated with

```shell
> make gir
```

When opening a PR please put the changes to the `src/auto` directory in a separate commit.

## License

__gtk__ is available under the MIT License, please refer to it.
