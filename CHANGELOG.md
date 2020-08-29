# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

> **Types of changes**:
>
> - **Added**: for new features.
> - **Changed**: for changes in existing functionality.
> - **Deprecated**: for soon-to-be removed features.
> - **Removed**: for now removed features.
> - **Fixed**: for any bug fixes.
> - **Security**: in case of vulnerabilities.

## [Unreleased]

## 0.2.0 -

### Added

 - Customizable threads number command
 - Customizable maximum waiting tasks command
 - Customizable maximum log actions command
 - Customizable maximum grids capacity command
 - Customizable ratio for BodyEmpty pixel
 - Customizable ratio for BodyBorder pixel
 - Clear Grid shortcut
 - Undo shortcut
 - Redo shortcut
 - Customizable texture format
 - Customizable Pixels-Cell ratio
 - Add Grid shortcut
 - Delete Grid shortcut
 - Switch grid tab shortcut
 - Switch grid order shortcut
 - Rename Grid shortcut
 - Parameters view

### Fixed

 - TasksManager can't shutdown its threadpool before the last thread ends.
 - Log doesn't overflow again after a decrementor/incrementor action.
 - Grid position for an odd terminal width doesn't pass over the workspace
block.
 - Grid position for an even terminal height doesn't pass over the workspace
block.

## 0.1.0 - 2020-08-12

Initial release
