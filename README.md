# Code Generator

* This project is not stabilized. There will be many breaking changes before it is stabilized.
* The goal of this project is to be able to generate code, and be able to easily change the formatting of the generated code as needed.
* This library makes it easier to reuse code generation code since it is context aware.
* This library currently only supports C, but I would like to support multiple target languages.

## TODO:

* Support more types of code generation (switch, do/while, etc.)
* Support more languages. Currently the language is fairly hard coded. Not sure if that can be abstracted.
* Have access to the end result of 'Name' types. Or create architecture to not need it.
* Unit tests
* Documentation
* Rework Names and add prepend/append operations
* Consider merging Composition, and Output phase into one. This would give an interface that sort of feels like an immediate mode gui type of structure.

### Breaking Changes from 0.1.x
* Names use a special character '`' to separate parts. They also consider capital characters to be the start of a new block, so "TEst" would be considered having parts ["T", "Est"].

## Limitations

* I don't believe it would be possible to work backwards with the existing architecture of the project. This library could not parse code, and do anything with it without a major rework.