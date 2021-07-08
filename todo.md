- If keyword! fails it will say an identifier is expected, which isn't true. It expected a keyword.
    How to fix:
    1. Add a Keyword variant to Token
    2. Make ParseToken a trait that can provide custom expected strings
- An attribute for Token variants, that allow you to provide special expected strings like "{" (results in expected "{")