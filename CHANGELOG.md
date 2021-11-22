# Changelog

## Unreleased

### Added

- Add html::push_html function.
- Support closing sequence on ATX-heading.
- Support carriage return.

### Fixed

- Leave trailing break at the end of text node in paragraph.
- Remove empty text node from ATX heading.
- Allow up to 3 spaces before ATX heading marker.

## 0.2.0

### Changed

- Change Event structure (e.g. ParagraphBegin -> Begin(Tag::Paragraph)).

## 0.1.1

### Added

- Initial release.
- Support paragraph.
- Support ATX heading.
