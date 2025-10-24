# 
# abstract-parser — proprietary, source-available software (not open-source).    
# Copyright (c) 2025 Abakar Letifov
# (Летифов Абакар Замединович). All rights reserved.
# 
# Use of this Work is permitted only for viewing and internal evaluation,        
# under the terms of the LICENSE file in the repository root.
# If you do not or cannot agree to those terms, do not use this Work.
# 
# THE WORK IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND.
# 

в данной грамматике PEG доступны следющие виды конструкций:

- конструкции токенов:
  - с выводом подстроки
    
    – после парсинга будет получена Span-подстрока.
    
    Пример:
    - `Ident = "[A-Za-z_]+"`
    - входная строка: "cpcl_ident"
    - вывод: "cpcl_ident"

  - с выводом указанноего типа
    
    – во время парсинга будет попытка преобразования полученной Span-подстрока в указанный тип. Тип должен быть `std::str::FromStr`.
    
    Пример:
    - `Number: f32 = "[0-9]+(?:\.[0-9]+)?"`
    - входная строка: "0.25"
    - вывод: 0.25_f32
  
  - без какого любого вывода
    
    – на практике используется, когда нам нужно лишь уcловное наличие правила в входной строке и не нужна выходная Span-подстрока.
    
    Пример:
    - `unit StartHeader = "start"`
    - входная строка: "start"
    - вывод: StartHeaderToken

- конструкции правил
    
    Любое правило имеет вид: `<RuleName><Generics>? = <Expr><OptionalEnding>`, где:
    - `<OptionalEnding>` присутствует **только** для `sequence` и принимает значение `";"`.
    - `RuleName = IDENT`
    - `Generics = "<" IDENT_1, IDENT_2, ... IDENT_N ">"`
    - `IDENT = "[A-Za-z_]+"`

    Expr:
    - combinatorExpr:
      - sequenceExpr это`<SubExpr1> <SubExpr2> ... <SubExprN>`
        , где `SubExpr = quantificatorExpr / (choice_orderingExpr)`
      - choice_orderingExpr это`<SubExpr1> / <SubExpr2> / ... / <SubExprN>`
        , где `SubExpr = quantificatorExpr / (sequenceExpr)`
    
    - quantificatorExpr (где `SubExpr = tokenExpr / (combinatorExpr)`):
        - Predicative:
            - Optional `<SubExpr>?`
            - NegativeLookahead `!<SubExpr>`
        
        - Kleene
            - ZeroOrMore `<SubExpr>*`
            - OneOrMore `<SubExpr>+`
        
        - RepeatQuantificator `<SubExpr><Repeat>`
            
            `Repeat` (где `N` и `K` это `"\d+"`):
            - Minimum `{N,}`
            - Maximum `{,K}`
            - MinMax `{N,K}`
            - Count `{N}`
        
        - Joinable это `<SubExpr> <JOINABLE_REPEAT><RepeatQuantificator>? <JoinableExpr>`
            , где:
            - `JOINABLE_REPEAT = "**"s`
            - `SubExpr` и `JoinableExpr` это `tokenExpr / (combinatorExpr)`
    
    - tokenExpr это `"<REG_EXPR>" / <RuleName><Generics>?`
        , где:
        - `REG_EXPR = "([^\"\\]|\\.)*"`
        - `Generics = "<" GenericExpr1, GenericExpr2, ... GenericExprN ">"`

    Примеры:
    - sequence
        `Assignment = Ident "=" Expression;`
        – ожидает идентификатор, затем символ `=`, затем выражение.
    
    - choice ordering
        `Literal = Number / String / Boolean`
        – если не удалось распарсить число, будет попытка строкового литерала, затем булевого.

    - Смешанный пример с choice, quantifier, joinable и sequence
        ```
        Expr<T> = Term<T> (("+" / "-") Term<T>)*;
        Term<T> = Factor<T> (("*" / "/") Factor<T>)*;
        Factor<T> = NUMBER / IDENTIFIER / "(" Expr<T> ")";
        ```

    - правило с параметрами
        ```
        List<T> = "[" T ("," T)* "]"
        ```
    - Joinable (**)
        ```
        CommaList = IDENTIFIER **{1,} ","
        ```
        – список из `IDENTIFIER` между которыми `","` минмум 1 раз (`{1,}`)

    - quantificator
        ```
        OptionalNumber = NUMBER?
        NotDigit = !"<[0-9]>"
        Words = "<[A-Za-z]+>"+
        Digits = "<[0-9]>*"
        FixedThree = "<[A-Z]>{3}"
        RangeRepeat = "<[A-Z]>{2,5}"
        ```

    - alias
        
        токены:
        ```
        AliasName = AnyToken
        ```
        ```
        AliasName = "[a-z][0-9]"
        ```
        с джинериками:
        ```
        AliasName = Head<Ident>
        ```
        ```
        AliasName<A, B> = Head<A> B;
        ```

- конструкции дерева
    - sequenceTree это `<RuleName><Generics>? <Body>`, где

        Body: 
        - named это `{ Field{2,} }`, где `Field = (IDENT "=" Expr) / Expr`. Поля без `IDENT` будут отсутвовать в выводе.
        
        - tuple это `( Field{2,} )`, где `Field = (#[ignore] Expr) / Expr`. Поля обозначенные `#[ignore]` будут отсутвовать в выводе.
        
        Примеры: для `Point (#[ignore] x, y)`, `Point {x, y_field: y}`, выражение `x` будет учавствовать в прасинге, но отсутсвовать в выводе.
    
    - choiceTree это `<RuleName><Generics>? ( <VariantName>(Expr) )`
        
        Пример:
        ```
        Expr (
            Literal("<[0-9]+>") /
            Binary(Expr "+" Expr) /
            Group("(" Expr ")")
        )
        ```