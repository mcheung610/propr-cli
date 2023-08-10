# Changelog

<a name="1.0.1" data-comment="this line is used by gitmoji-changelog, don't remove it!"></a>
## Version 1.0.1

### Changed
- 🔧  Slightly improve compile speed

### Miscellaneous
- 📝  Update CHANGELOG

<a name="1.0.0" data-comment="this line is used by gitmoji-changelog, don't remove it!"></a>
## Version 1.0.0

### Changed
- 🚸  Match the system message with propr.dev

### Miscellaneous
- 📝  Add info about generating the title
- 📝  Give some information when to use generate
- 📝  Update create usage
- 📝  Update CHANGELOG

<a name="0.9.5" data-comment="this line is used by gitmoji-changelog, don't remove it!"></a>
## Version 0.9.5

### Removed
- 🔥  Remove reference to developer since it sometimes gets mentioned in the response

### Miscellaneous
- 📝  Update CHANGELOG

<a name="0.9.4" data-comment="this line is used by gitmoji-changelog, don't remove it!"></a>
## Version 0.9.4

### Miscellaneous
- ⚗  Change the system message slightly to give better results
- 📝  Update CHANGELOG

<a name="0.9.3" data-comment="this line is used by gitmoji-changelog, don't remove it!"></a>
## Version 0.9.3

### Changed
- 🎨  Rename for better readability
- ♻️  Move unwrapping and serializing down a level
- ♻️  Move context of prompt to system message

### Miscellaneous
- 📝  Update CHANGELOG

<a name="0.9.2" data-comment="this line is used by gitmoji-changelog, don't remove it!"></a>
## Version 0.9.2

### Miscellaneous
- ⚗  Experiment with a slightly different prompt
- 📝  Update CHANGELOG

<a name="0.9.1" data-comment="this line is used by gitmoji-changelog, don't remove it!"></a>
## Version 0.9.1

### Added
- ✨  Also expose model flag when generating

### Miscellaneous
- 📝  Update CHANGELOG

<a name="0.9.0" data-comment="this line is used by gitmoji-changelog, don't remove it!"></a>
## Version 0.9.0

### Added
- ✨  Allow specifying model as a flag

### Miscellaneous
- 📝  Update README
- 📝  Update CHANGELOG

<a name="0.8.0" data-comment="this line is used by gitmoji-changelog, don't remove it!"></a>
## Version 0.8.0

### Added
- ✨  Add more model options

### Miscellaneous
- 📝  Update CHANGELOG

<a name="0.7.0" data-comment="this line is used by gitmoji-changelog, don't remove it!"></a>
## Version 0.7.0

### Added
- ✨  Allow configuring base branch

### Miscellaneous
- 📝  Update CHANGELOG

<a name="0.6.2" data-comment="this line is used by gitmoji-changelog, don't remove it!"></a>
## Version 0.6.2

### Changed
- ♻️  Handle specific errors separately

### Fixed
- 🐛  Properly handle invalid API responses

### Miscellaneous
- 📝  Update CHANGELOG

<a name="0.6.1" data-comment="this line is used by gitmoji-changelog, don't remove it!"></a>
## Version 0.6.1

### Added
- 👷  Update rust version used when building

### Fixed
- 🐛  Fetch diff in comparison with origin not local HEAD

### Miscellaneous
- 📝  Update CHANGELOG

<a name="0.6.0" data-comment="this line is used by gitmoji-changelog, don't remove it!"></a>
## Version 0.6.0

### Added
- ✨  Introduce human readable panic message
- ➕  Add human_panic
- 👷  Update GitHub Actions workflows and improve release process

### Changed
- ⬆️  Bump deps
- ♻️  Refactor loader messages and spinner style

### Removed
- 🔥  Remove as it is being ignored anyway

### Fixed
- 💚  Release needs build

### Miscellaneous
- 💡  Add some descriptive comments
- 📝  Update CHANGELOG

<a name="0.5.0" data-comment="this line is used by gitmoji-changelog, don't remove it!"></a>
## Version 0.5.0

### Added
- ✨  Add option to configure automatic PR title generation

### Changed
- 🚨  Fix linting
- 🎨  Clean up for readability

### Miscellaneous
- 📝  Update CHANGELOG

<a name="0.4.0" data-comment="this line is used by gitmoji-changelog, don't remove it!"></a>
## Version 0.4.0

### Added
- ✨  Implement automatic PR title generation using OpenAI API
- ✨  Keep generating until user confirms

### Changed
- ♻️  Ask title before generating

### Removed
- 🔥  Remove RUSTFLAGS

### Miscellaneous
- 📝  Update CHANGELOG

<a name="0.3.1" data-comment="this line is used by gitmoji-changelog, don't remove it!"></a>
## Version 0.3.1

### Changed
- 🔨  Remove changelog before updating

### Fixed
- 🐛  Fix error handling in create command

### Miscellaneous
- 📝  Update CHANGELOG

<a name="0.3.0" data-comment="this line is used by gitmoji-changelog, don't remove it!"></a>
## Version 0.3.0

### Added
- ✨  Ask user for title

### Miscellaneous
- 📝  Update CHANGELOG

<a name="0.2.0" data-comment="this line is used by gitmoji-changelog, don't remove it!"></a>
## Version 0.2.0

### Added
- ✨  Add option to configure model to use

### Changed
- 🔨  Add some extra optims to improve binary
- 🔨  Apparantly awk doesn't work as expected in make
- 🔨  Make sure we extract correct version
- ♻️  Use confy for everything, drop the custom config stuff

### Miscellaneous
- 📝  Update CHANGELOG

<a name="0.1.0" data-comment="this line is used by gitmoji-changelog, don't remove it!"></a>
## Version 0.1.0

### Added
- ✨  Allow using gpt-4 over gpt-3.5-turbo
- 👷  Make sure we build on PRs as well so we guarantee that it keeps working
- 🎉  Add initial setup for propr cli usage

### Changed
- 🔨  Downgraded to 0.3.0 so now this should work
- 🔨  Write to CHANGELOG.md
- 🔨  Force the generation
- 🔨  Use global install
- ♻️  Properly check if env var is set
- 🎨  Rewrite to improve readability

### Fixed
- 🐛  Make sure we also apply correct head when determining base commit
- 🐛  Make sure we don't clear the template
- 🐛  Fix git diff command in get_diff function

### Miscellaneous
- 📝  Update CHANGELOG
- 📝  Add CHANGELOG
- 📝  Add README

_Generated by [gitmoji-changelog (rust version)](https://github.com/fabienjuif/gitmoji-changelog-rust)_