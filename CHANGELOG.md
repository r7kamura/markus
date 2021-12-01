# Changelog

## Unreleased

### Added

- Support HTML blocks.

## 0.6.0 - 2021-11-28

### Added

- Support indented code blocks.
- Support fenced code blocks.

## 0.5.0 - 2021-11-25

### Added

- Support setext headings.

### Fixed

- No skip non-space-prefixed enclosing sequence at ATX heading.

## 0.4.1 - 2021-11-23

### Fixed

- Skip leading spaces or tabs in paragraph.
- Skip leading spaces or tabs of block.
- Interrupt paragraph by ATX heading.

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
