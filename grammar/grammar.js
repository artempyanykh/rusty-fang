const arrow_tok = '->'
const lambda_tok = '\\'
const newline_tok = '\n'

const mul_op = /\*|\//
const add_op = /\+|\-/
const rel_op = /<=?|>=?|==/
const pref_op = choice('-')

module.exports = grammar({
    name: 'fang',

    word: $ => $.identifier,

    rules: {
        unit: $ => optional(seq(
            $._expression,
            repeat(seq(newline_tok, $._expression)),
            optional(newline_tok),
        )),

        _expression: $ => choice(
            seq('(', $._expression, ')'),
            $.const_int,
            $.const_bool,
            $.identifier,
            $.infix_ex,
            $.prefix_ex,
            $.let,
            $.binding,
            $.lambda,
            $.ap,
            $.cond,
        ),

        infix_ex: $ => {
            return choice(
                ...[[mul_op, 3], [add_op, 2], [rel_op, 1]].map(([op, op_prec]) =>
                    prec.left(op_prec, seq(
                        field('lhs', $._expression),
                        field('op', alias(op, $.infix_op)),
                        field('rhs', $._expression)
                    ))
                )
            )
        },

        infix_op: $ => choice(
            mul_op,
            add_op,
            rel_op
        ),

        prefix_ex: $ => prec.left(4,
            seq(
                field('op', alias(pref_op, $.prefix_op)),
                field('body', $._expression),
            )
        ),

        prefix_op: $ => choice(
            pref_op
        ),

        binding: $ => seq(
            field('lhs', $.identifier),
            field('params', repeat($.identifier)),
            '=',
            field('rhs', $._expression),
        ),

        let: $ => seq(
            'let',
            field('bindings', $._binding_list),
            'in',
            field('body', $._expression),
        ),

        _binding_list: $ => prec.left(seq(
            $.binding,
            repeat(
                seq(
                    ',',
                    $.binding
                )
            )

        )),

        lambda: $ => seq(
            lambda_tok,
            field('params', repeat($.identifier)),
            arrow_tok,
            field('body', $._expression),
        ),

        ap: $ => prec.left(5,
            seq(
                field('receiver', $._expression),
                '(',
                field('arguments', optional($._param_list)),
                ')'
            )
        ),

        _param_list: $ => seq(
            $._expression,
            repeat(seq(',', $._expression)),
            optional(',')
        ),


        cond: $ => seq(
            'if',
            field('pred', $._expression),
            'then',
            field('then', $._expression),
            'else',
            field('else', $._expression),
        ),

        identifier: $ => /_?[A-Za-z][\w']*/,

        const_int: $ => /\-?\d[\d_]*/,

        const_bool: $ => choice("True", "False"),

    }
});