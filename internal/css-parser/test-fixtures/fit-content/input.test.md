# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/css-parser/index.test.ts --update-snapshots` to update.

## `fit-content`

### `ast`

```javascript
CSSRoot {
	body: [
		CSSRule {
			prelude: [
				CSSSelector {
					patterns: [
						CSSClassSelector {
							value: "style"
							loc: SourceLocation fit-content/input.css 1:0-1:6
						}
					]
					loc: SourceLocation fit-content/input.css 1:0-1:7
				}
			]
			block: CSSBlock {
				value: [
					CSSDeclaration {
						name: "grid-template-columns"
						value: [
							CSSFitContentFunction {
								name: "fit-content"
								params: [
									CSSDimension {
										value: 8
										unit: "ch"
										loc: SourceLocation fit-content/input.css 2:36-2:39
									}
								]
								loc: SourceLocation fit-content/input.css 2:24-2:40
							}
							CSSFitContentFunction {
								name: "fit-content"
								params: [
									CSSDimension {
										value: 8
										unit: "ch"
										loc: SourceLocation fit-content/input.css 2:53-2:56
									}
								]
								loc: SourceLocation fit-content/input.css 2:41-2:57
							}
							CSSDimension {
								value: 1
								unit: "fr"
								loc: SourceLocation fit-content/input.css 2:58-2:61
							}
						]
						important: false
						loc: SourceLocation fit-content/input.css 2:1-2:61
					}
				]
				startingTokenValue: "{"
				loc: SourceLocation fit-content/input.css 1:7-3:1
			}
			loc: SourceLocation fit-content/input.css 1:0-3:1
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	path: UIDPath<fit-content/input.css>
	loc: SourceLocation fit-content/input.css 1:0-3:1
}
```

### `diagnostics`

```

```