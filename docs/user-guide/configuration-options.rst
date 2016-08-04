Configuration options
=====================

Webkitten supports several configuration options relating to content filtering,
browser capability, appearance, and command execution. These options are the
basis of any Webkitten-based browser.

Commands may define additional options, but these are supported by default.

General
-------

Base defaults for window appearance and new web view buffer behavior.

.. glossary::

   general.allow-javascript
     If ``true``, JavaScript will be permitted to run within new web view
     buffers. If unset, this value defaults to ``true``.

   general.allow-plugins
     If ``true``, browser plugins such as Silverlight and Flash will be
     permitted to load. If unset, this value defaults to ``false``.

   general.bar-font
     A pair of values, ``size`` and ``family``, which represent the font to be
     used in the command bar. If unset, font preference is left to the GUI
     binding implementation.

   general.config-dir
     The configuration directory which can be substituted with ``CONFIG_DIR``
     within other options requiring file paths

   general.content-filter
     A path to a file containing content filtering rules to be applied by
     default. If unset, no content filtering is applied.

   general.private-browsing
     If ``true``, new web view buffers are opened in private browsing mode by
     default. No browsing history or content can be persisted from these
     sessions. If unset, this value defaults to ``false``.

   general.skip-content-filter
     If ``true``, the content filter file is not applied to new web view
     buffers.

   general.start-page
     A file or HTTP url indicating what content should be loaded in new web
     view buffers.

Commands
--------

Options regarding command loading, event triggers, and shortcuts.

.. glossary::

   commands.aliases."[ALIAS]"
     A command name to be invoked when the command bar text matches ``[ALIAS]``

   commands.default
     The command invoked when no command files are found matching the first
     word

   commands.disabled
     Disabled commands by name, which are skipped when resolving commands

   commands.keybindings."[COMMAND]"
     A key chord representation which should invoke ``[COMMAND]`` when pressed.
     Each chord is represented by a combination of ``super``/``command``,
     ``ctrl``, ``alt``/``option``, and ``shift``, combined with a single
     character and separated by spaces. I.e., ``cmd shift n``.

   commands.on-fail-uri
     An array of command names to invoke when a resource fails to load

   commands.on-load-uri
     An array of command names to invoke when a resource loads

   commands.on-request-uri
     An array of command names to invoke when a resource is requested

   commands.on-text-change."[CHAR]"
     A command name to invoke as text changes in the command bar while the
     first character is ``[CHAR]``.

   commands.search-paths
     An array of string paths used to search for command files

Site-specific options
---------------------

General configuration options regarding web view behavior can be overridden
when opening a buffer by following a link to a host and defining site-specific
configuration options. These options are:

.. glossary::

   sites."[HOST]".general.allow-javascript
     If ``true``, any new buffers opened while linking to ``[HOST]`` will
     enable JavaScript to run.

   sites."[HOST]".general.allow-plugins
     If ``true``, any new buffers opened while linking to ``[HOST]`` will
     enable browser plugins such as Silverlight and Flash.

   sites."[HOST]".general.private-browsing
     If ``true``, any new buffers opened while linking to ``[HOST]`` will
     enable private browsing.

   sites."[HOST]".general.skip-content-filter
     If ``true``, any new buffers opened while linking to ``[HOST]`` will
     not load the content filter file.
