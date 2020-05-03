digit = /\d/
arrow = '->'
lambda = '\\'

module.exports = grammar({
    name: 'fang',

    word: $ => $.identifier,

    rules: {
        unit: $ => repeat($._expression),

        _expression: $ => choice(
            $.integer,
            $.bool,
            $.identifier,
            $.binding,
            $.lambda,
        ),

        binding: $ => seq(
            $.identifier,
            '=',
            $._expression,
        ),

        lambda: $ => seq(
            lambda,
            field('bound', repeat($.identifier)),
            arrow,
            field('body', $._expression),
        ),

        identifier: $ => /_?[A-Za-z]\w*/,

        integer: $ => /\-?(\d_?)*\d+/,

        bool: $ => choice("True", "False"),

    }
});