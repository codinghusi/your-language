# Documentation
Here is an [example](./target-syntax)

## Nodes
All parts of the languages' code are cleverly extracted as Nodes, that are part of the AST.

## Nodes' description
Define a node that parses e.g. a variable, function or class name:
```yl
node Identifier {
    describe() => value: /[_a-zA-Z]\w*/;
}
```

The AST Result for the code `myVeryNiceIdentifier` would be:
```json
{
    "type": "IdentifierNode",
    "value": "myVeryNiceIdentifier"
}
```

So for better understanding here more generalized:
```yl
node YourNodesName {
    describe() => /* How should I parse a code segment? */;
}
```
Read more below about the parsing syntax.
### Cases
#### Error handling
```yl
node Identifier {
    describe() => value: /\w+/;
    
    case(value.matches(/^\d/)) {
        error("An identifier mustn't start with a number");
    }
}
```
To handle errors, Your Language takes advantages of cases. Think them as if statements.
```yl
node Whatever {
    ...
    case (condition) {
        // throw error, warning, help
        // or provide more details about the node for analysis purpose 
    }
}
```

#### More
Like error(...) there's warning(...). The function help(...) provides info to the programmer, how he could fix the issue.
TODO: how to provide details for analysis?


### Parsing Syntax
The syntax is based on different patterns joined by whitespace operators.
So here's an example that parses the code `let foo = "bar"`:
```yl
node VariableDeclaration {
    describe() => "let" -!> name: Identifier() -> "=" -> init: Expression();
}
```
The actual parsing is that one:
`"let" -!> name: Identifier() -> "=" -> init: Expression()`
And here's a step by step explaination what it does:

| code                 | description                                                                                             | details             |
|----------------------|---------------------------------------------------------------------------------------------------------|---------------------|
| `let`                | Eat the next three chars that must be 'l', 'e', 't'                                                     | StringEater         |
| `-!>`                  | Require a whitespace                                                                                    | Whitespace operands |
| `name: Identifier()` | Parse an Identifier using the IdentifierNode (declared in examples above) and save its result as "name" | NodeEater           |
| `->`                  | Allow a whitespace                                                                                      | Whitespace operands |
| `init: Expression()` | Parse an expression (also defined as a node) and store its value as "init"                              | NodeEater           |

#### Whitespace operators
| whitespace  | following optional | operator |
|-------------|--------------------|----------|
| optional    | no                 | ->       |
| required    | no                 | ->>      |
| not allowed | no                 | -!>      |
| optional    | yes                | ~>       |
| required    | yes                | ~>>      |
| not allowed | yes                | ~!>      |


#### StringEater
Define a string that's expected  
Example:  
`"let"`

#### RegexEater
Parses the given regex, encapsulated by /.../ and flags behind the last slash (syntax like in JavaScript)  
Example:  
`/[a-z_]+/i`

#### NodeEater
Parses with the given node and returns the actual AST result, optionally you can pass arguments (arguments currently aren't thought through).  
Example:  
`Identifier()`

#### ValueCaputure
You want to save some results of parsers. You do that by writing the key with a colon in front of the eater.
It is possible to capture whitespaces.
Keys with a $ sign in front, won't affect the AST result. These are used as variables for things like error checking  
Example:  
`mykey: SomethingToParse()`

## Relations
Relations are actual aliases on nodes.
These are just for analyze purposes.  
Example:  
```yl
relation Variable on Identifier {}
relation Function on Identifier {}

node VariableDeclaration {
    describe() => "let" -!> Variable();
}

node FunctionDeclaration {
    describe() => "function -!> Function() -> "()" -> CodeBlock();
}
```
Later when I parse Identifier(), my analyser can identify, if it is a **variable** or a **function**.

## Grouping / Namespace
Removes boilerplate code, where you config stuff for all group members.

### Nodes
```yl
nodes Expression {
    node Binary {
        ...
    }
    node Call {
        ...
    }
}
```
You can parse Binary using `Expression::Binary()` but also try both Binary and Call, if you simply parse using `Expression()`

### Relations
```yl
relations Variable on Identifier {
    relation Let { }
    relation Const { }
}
```
Sets the relation base once on top, in the `relation` statements it can be omitted.
Traits are also possible in the root level.

## Traits
TODO: fill