# Copyright © SixtyFPS GmbH <info@slint.dev>
# SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Software-3.0

# Configuration file for the Sphinx documentation builder.
#
# This file only contains a selection of the most common options. For a full
# list see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Path setup --------------------------------------------------------------

# If extensions (or modules to document with autodoc) are in another directory,
# add these directories to sys.path here. If the directory is relative to the
# documentation root, use os.path.abspath to make it absolute, like shown here.
#
import os
import sys

sys.path.insert(0, os.path.abspath("."))
import textwrap


# -- Project information -----------------------------------------------------

# The full version, including alpha/beta/rc tags
version = "1.9.0"

project = f"Slint C++ API"
html_title = f"Slint {version} C++ API"  # Set title here, otherwise it will say "Slint Reference documentation"
copyright = "SixtyFPS GmbH"
author = "Slint Developers <info@slint.dev>"
github_url = "https://github.com/slint-ui/slint"

cpp_index_common_prefix = ["slint::", "slint::interpreter::"]

# -- General configuration ---------------------------------------------------

# Add any Sphinx extension module names here, as strings. They can be
# extensions coming with Sphinx (named 'sphinx.ext.*') or your custom
# ones.
extensions = [
    "breathe",
    "exhale",
    "myst_parser",
    "sphinx_markdown_tables",
    "sphinx.ext.autosectionlabel",
    "sphinxcontrib.jquery",
    "sphinx_tabs.tabs",
    "sphinx_design",
    "sphinx_copybutton",
    "sphinx_sitemap",
]

breathe_projects = {"Slint": "./docs/xml"}
breathe_default_project = "Slint"

exhale_args = {
    "containmentFolder": "./api",
    "rootFileName": "library_root.rst",
    "rootFileTitle": "C++ API Reference",
    "afterTitleDescription": textwrap.dedent(
        """
            The following sections present the C++ API Reference. All types are
            within the :ref:`slint<namespace_slint>` namespace and are accessible by including
            the :code:`slint.h` header file.

            If you choose to load :code:`.slint` files dynamically at run-time, then
            you can use the classes in :ref:`slint::interpreter<namespace_slint__interpreter>`, starting at
            :cpp:class:`slint::interpreter::ComponentCompiler`. You need to include
            the :code:`slint-interpreter.h` header file.
        """
    ),
    "doxygenStripFromPath": "..",
    "createTreeView": True,
    "kindsWithContentsDirectives": [],
    "exhaleExecutesDoxygen": True,
    "exhaleDoxygenStdin": """INPUT = ../../api/cpp/include generated_include
EXCLUDE_SYMBOLS = slint::cbindgen_private* slint::private_api* vtable* SLINT_DECL_ITEM
EXCLUDE = ../../api/cpp/include/vtable.h ../../api/cpp/include/slint_tests_helper.h ../../api/cpp/include/slint-stm32.h
ENABLE_PREPROCESSING = YES
PREDEFINED += DOXYGEN
INCLUDE_PATH = generated_include
WARN_AS_ERROR = YES""",
}

# Add any paths that contain templates here, relative to this directory.
templates_path = ["../../../docs/reference/_templates"]

# List of patterns, relative to source directory, that match files and
# directories to ignore when looking for source files.
# This pattern also affects html_static_path and html_extra_path.
exclude_patterns = [
    "_build",
    "html/_static/collapsible-lists/LICENSE.md",
    "Thumbs.db",
    ".DS_Store",
    "markdown/tutorial",
    "markdown/building.md",
    "markdown/development.md",
    "markdown/install_qt.md",
    "markdown/README.md",
    "README.md",
]


# -- Options for HTML output -------------------------------------------------

# The theme to use for HTML and HTML Help pages.  See the documentation for
# a list of builtin themes.
#
html_theme = "sphinx_book_theme"
html_favicon = "https://slint.dev/favicon.svg"
html_theme_options = {
    "repository_url": "https://github.com/slint-ui/slint",
    "use_repository_button": True,
    "use_issues_button": True,
    "use_edit_page_button": False,
    "use_download_button": False,
    "use_fullscreen_button": False,
    "home_page_in_toc": True,
    "logo": {
        "text": f"Slint {version} C++ API",
        "image_light": "https://slint.dev/logo/slint-logo-small-light.svg",
        "image_dark": "https://slint.dev/logo/slint-logo-small-dark.svg",
        "link": "https://slint.dev",
    },
    "switcher": {
        "json_url": "https://releases.slint.dev/versions.json",
        "version_match": version,
    },
    "extra_footer": '<div><a href="https://slint.dev">https://slint.dev</a></div>',
    "article_header_start": ["toggle-primary-sidebar.html"],
    "article_header_end": ["searchbox.html", "article-header-buttons.html"],
    "show_version_warning_banner": True,
}

html_baseurl = "https://docs.slint.dev/"
sitemap_url_scheme = "master/docs/slint/{link}"
html_sidebars = {"**": ["version-switcher", "navbar-logo.html", "sbt-sidebar-nav.html"]}

# Add any paths that contain custom static files (such as style sheets) here,
# relative to this directory. They are copied after the builtin static files,
# so a file named "default.css" will overwrite the builtin "default.css".
html_static_path = ["../../../docs/reference/_static"]
html_js_files = ["cm6.bundle.js", "expand_tabs.js"]
html_css_files = [
    "theme_tweak.css",
    "https://cdn.jsdelivr.net/npm/typesense-docsearch-css@0.3.0",
]
html_show_sourcelink = False

myst_enable_extensions = ["html_image", "colon_fence", "linkify"]

myst_url_schemes = {
    "slint-qs": f"https://slint.dev/releases/{version}/docs/quickstart/{{{{path}}}}",
    "slint-cpp": f"https://slint.dev/releases/{version}/docs/cpp/{{{{path}}}}",
    "slint-rust": f"https://slint.dev/releases/{version}/docs/rust/slint/{{{{path}}}}",
    "slint-build-rust": f"https://slint.dev/releases/{version}/docs/rust/slint_build/{{{{path}}}}",
    "slint-node": f"https://slint.dev/releases/{version}/docs/node/{{{{path}}}}",
    "slint-reference": f"https://slint.dev/releases/{version}/docs/slint/{{{{path}}}}",
    "http": None,
    "https": None,
    "mailto": None,
}

# Annotate h1/h2 elements with anchors
myst_heading_anchors = 4

rst_epilog = """
"""
