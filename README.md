Operator = 
    | +
    | -
    | *
    | /
    | ()

Digit = 
    | 0 ... 9

Integer =
    | -2^63
    | ...
    | 1
    | ...
    | 2^63-1

Identifier =
    | a ... z
    | A ... Z
    | Identifier||Identifier
    | Identifier||Digit

FunctionBody =
    |

Function = 
    | 'fn '||Identifier||'('||Identifier*||')'||' '*||
        '{'||FunctionBody||'}'

Expression =
    | Function||(Expression||Expression)*||';'
    | Term||(Operator||Term)*||';'

Subexpression =
    | '('||Expression||')'

Factor =
    | Integer
    | Identifier
    | Subexpression

Term =
    | Factor
    | Factor||(Operator||Factor)*
