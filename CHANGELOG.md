# Changelog

## Unreleased

### Fixed

- Skip leading spaces or tabs in paragraph.
- Skip leading spaces or tabs of block.

## 0.4.0 - 2021-11-23

### Added

- Support thematic breaks.
- Support paragraph interrupt.

### Changed

- Append line feed after closing HTML tag.

### Fixed

- Fix max preceding spaces length on ATX headings.
- Remove trailing line ending from paragraph last text node.

## 0.3.0 - 2021-11-22

### Added

- Add html::push_html function.
- Support closing sequence on ATX-heading.
- Support carriage return.

### Fixed

- Leave trailing break at the end of text node in paragraph.
- Remove empty text node from ATX heading.
- Allow up to 3 spaces before ATX heading marker.

## 0.2.0 - 2021-11-22

### Changed

- Change Event structure (e.g. ParagraphBegin -> Begin(Tag::Paragraph)).

## 0.1.1 - 2021-11-22

### Added

- Initial release.
- Support paragraph.
- Support ATX heading.
