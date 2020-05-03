#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 11
#define STATE_COUNT 16
#define LARGE_STATE_COUNT 7
#define SYMBOL_COUNT 15
#define ALIAS_COUNT 0
#define TOKEN_COUNT 8
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 2
#define MAX_ALIAS_SEQUENCE_LENGTH 4

enum {
  sym_identifier = 1,
  anon_sym_EQ = 2,
  anon_sym_BSLASH = 3,
  anon_sym_DASH_GT = 4,
  sym_integer = 5,
  anon_sym_True = 6,
  anon_sym_False = 7,
  sym_unit = 8,
  sym__expression = 9,
  sym_binding = 10,
  sym_lambda = 11,
  sym_bool = 12,
  aux_sym_unit_repeat1 = 13,
  aux_sym_lambda_repeat1 = 14,
};

static const char *ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_identifier] = "identifier",
  [anon_sym_EQ] = "=",
  [anon_sym_BSLASH] = "\\",
  [anon_sym_DASH_GT] = "->",
  [sym_integer] = "integer",
  [anon_sym_True] = "True",
  [anon_sym_False] = "False",
  [sym_unit] = "unit",
  [sym__expression] = "_expression",
  [sym_binding] = "binding",
  [sym_lambda] = "lambda",
  [sym_bool] = "bool",
  [aux_sym_unit_repeat1] = "unit_repeat1",
  [aux_sym_lambda_repeat1] = "lambda_repeat1",
};

static TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_identifier] = sym_identifier,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_BSLASH] = anon_sym_BSLASH,
  [anon_sym_DASH_GT] = anon_sym_DASH_GT,
  [sym_integer] = sym_integer,
  [anon_sym_True] = anon_sym_True,
  [anon_sym_False] = anon_sym_False,
  [sym_unit] = sym_unit,
  [sym__expression] = sym__expression,
  [sym_binding] = sym_binding,
  [sym_lambda] = sym_lambda,
  [sym_bool] = sym_bool,
  [aux_sym_unit_repeat1] = aux_sym_unit_repeat1,
  [aux_sym_lambda_repeat1] = aux_sym_lambda_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BSLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH_GT] = {
    .visible = true,
    .named = false,
  },
  [sym_integer] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_True] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_False] = {
    .visible = true,
    .named = false,
  },
  [sym_unit] = {
    .visible = true,
    .named = true,
  },
  [sym__expression] = {
    .visible = false,
    .named = true,
  },
  [sym_binding] = {
    .visible = true,
    .named = true,
  },
  [sym_lambda] = {
    .visible = true,
    .named = true,
  },
  [sym_bool] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_unit_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_lambda_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum {
  field_body = 1,
  field_bound = 2,
};

static const char *ts_field_names[] = {
  [0] = NULL,
  [field_body] = "body",
  [field_bound] = "bound",
};

static const TSFieldMapSlice ts_field_map_slices[3] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 2},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_body, 2},
  [1] =
    {field_body, 3},
    {field_bound, 1},
};

static TSSymbol ts_alias_sequences[3][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(4);
      if (lookahead == '-') ADVANCE(1);
      if (lookahead == '=') ADVANCE(5);
      if (lookahead == '\\') ADVANCE(6);
      if (lookahead == '_') ADVANCE(3);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(9);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(8);
      END_STATE();
    case 1:
      if (lookahead == '>') ADVANCE(7);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(9);
      END_STATE();
    case 2:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(9);
      END_STATE();
    case 3:
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(8);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(anon_sym_BSLASH);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(anon_sym_DASH_GT);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(8);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(sym_integer);
      if (lookahead == '_') ADVANCE(2);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(9);
      END_STATE();
    default:
      return false;
  }
}

static bool ts_lex_keywords(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (lookahead == 'F') ADVANCE(1);
      if (lookahead == 'T') ADVANCE(2);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      END_STATE();
    case 1:
      if (lookahead == 'a') ADVANCE(3);
      END_STATE();
    case 2:
      if (lookahead == 'r') ADVANCE(4);
      END_STATE();
    case 3:
      if (lookahead == 'l') ADVANCE(5);
      END_STATE();
    case 4:
      if (lookahead == 'u') ADVANCE(6);
      END_STATE();
    case 5:
      if (lookahead == 's') ADVANCE(7);
      END_STATE();
    case 6:
      if (lookahead == 'e') ADVANCE(8);
      END_STATE();
    case 7:
      if (lookahead == 'e') ADVANCE(9);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(anon_sym_True);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(anon_sym_False);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
};

static uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_BSLASH] = ACTIONS(1),
    [anon_sym_DASH_GT] = ACTIONS(1),
    [sym_integer] = ACTIONS(1),
    [anon_sym_True] = ACTIONS(1),
    [anon_sym_False] = ACTIONS(1),
  },
  [1] = {
    [sym_unit] = STATE(15),
    [sym__expression] = STATE(2),
    [sym_binding] = STATE(2),
    [sym_lambda] = STATE(2),
    [sym_bool] = STATE(2),
    [aux_sym_unit_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(3),
    [sym_identifier] = ACTIONS(5),
    [anon_sym_BSLASH] = ACTIONS(7),
    [sym_integer] = ACTIONS(9),
    [anon_sym_True] = ACTIONS(11),
    [anon_sym_False] = ACTIONS(11),
  },
  [2] = {
    [sym__expression] = STATE(3),
    [sym_binding] = STATE(3),
    [sym_lambda] = STATE(3),
    [sym_bool] = STATE(3),
    [aux_sym_unit_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(13),
    [sym_identifier] = ACTIONS(5),
    [anon_sym_BSLASH] = ACTIONS(7),
    [sym_integer] = ACTIONS(15),
    [anon_sym_True] = ACTIONS(11),
    [anon_sym_False] = ACTIONS(11),
  },
  [3] = {
    [sym__expression] = STATE(3),
    [sym_binding] = STATE(3),
    [sym_lambda] = STATE(3),
    [sym_bool] = STATE(3),
    [aux_sym_unit_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(17),
    [sym_identifier] = ACTIONS(19),
    [anon_sym_BSLASH] = ACTIONS(22),
    [sym_integer] = ACTIONS(25),
    [anon_sym_True] = ACTIONS(28),
    [anon_sym_False] = ACTIONS(28),
  },
  [4] = {
    [sym__expression] = STATE(9),
    [sym_binding] = STATE(9),
    [sym_lambda] = STATE(9),
    [sym_bool] = STATE(9),
    [sym_identifier] = ACTIONS(5),
    [anon_sym_BSLASH] = ACTIONS(7),
    [sym_integer] = ACTIONS(31),
    [anon_sym_True] = ACTIONS(11),
    [anon_sym_False] = ACTIONS(11),
  },
  [5] = {
    [sym__expression] = STATE(10),
    [sym_binding] = STATE(10),
    [sym_lambda] = STATE(10),
    [sym_bool] = STATE(10),
    [sym_identifier] = ACTIONS(5),
    [anon_sym_BSLASH] = ACTIONS(7),
    [sym_integer] = ACTIONS(33),
    [anon_sym_True] = ACTIONS(11),
    [anon_sym_False] = ACTIONS(11),
  },
  [6] = {
    [sym__expression] = STATE(11),
    [sym_binding] = STATE(11),
    [sym_lambda] = STATE(11),
    [sym_bool] = STATE(11),
    [sym_identifier] = ACTIONS(5),
    [anon_sym_BSLASH] = ACTIONS(7),
    [sym_integer] = ACTIONS(35),
    [anon_sym_True] = ACTIONS(11),
    [anon_sym_False] = ACTIONS(11),
  },
};

static uint16_t ts_small_parse_table[] = {
  [0] = 3,
    ACTIONS(41), 1,
      anon_sym_EQ,
    ACTIONS(37), 3,
      ts_builtin_sym_end,
      anon_sym_BSLASH,
      sym_integer,
    ACTIONS(39), 3,
      sym_identifier,
      anon_sym_True,
      anon_sym_False,
  [14] = 2,
    ACTIONS(43), 3,
      ts_builtin_sym_end,
      anon_sym_BSLASH,
      sym_integer,
    ACTIONS(45), 3,
      sym_identifier,
      anon_sym_True,
      anon_sym_False,
  [25] = 2,
    ACTIONS(47), 3,
      ts_builtin_sym_end,
      anon_sym_BSLASH,
      sym_integer,
    ACTIONS(49), 3,
      sym_identifier,
      anon_sym_True,
      anon_sym_False,
  [36] = 2,
    ACTIONS(51), 3,
      ts_builtin_sym_end,
      anon_sym_BSLASH,
      sym_integer,
    ACTIONS(53), 3,
      sym_identifier,
      anon_sym_True,
      anon_sym_False,
  [47] = 2,
    ACTIONS(55), 3,
      ts_builtin_sym_end,
      anon_sym_BSLASH,
      sym_integer,
    ACTIONS(57), 3,
      sym_identifier,
      anon_sym_True,
      anon_sym_False,
  [58] = 3,
    ACTIONS(59), 1,
      sym_identifier,
    ACTIONS(61), 1,
      anon_sym_DASH_GT,
    STATE(13), 1,
      aux_sym_lambda_repeat1,
  [68] = 3,
    ACTIONS(63), 1,
      sym_identifier,
    ACTIONS(65), 1,
      anon_sym_DASH_GT,
    STATE(14), 1,
      aux_sym_lambda_repeat1,
  [78] = 3,
    ACTIONS(67), 1,
      sym_identifier,
    ACTIONS(70), 1,
      anon_sym_DASH_GT,
    STATE(14), 1,
      aux_sym_lambda_repeat1,
  [88] = 1,
    ACTIONS(72), 1,
      ts_builtin_sym_end,
};

static uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(7)] = 0,
  [SMALL_STATE(8)] = 14,
  [SMALL_STATE(9)] = 25,
  [SMALL_STATE(10)] = 36,
  [SMALL_STATE(11)] = 47,
  [SMALL_STATE(12)] = 58,
  [SMALL_STATE(13)] = 68,
  [SMALL_STATE(14)] = 78,
  [SMALL_STATE(15)] = 88,
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.count = 0, .reusable = false},
  [1] = {.count = 1, .reusable = false}, RECOVER(),
  [3] = {.count = 1, .reusable = true}, REDUCE(sym_unit, 0),
  [5] = {.count = 1, .reusable = false}, SHIFT(7),
  [7] = {.count = 1, .reusable = true}, SHIFT(12),
  [9] = {.count = 1, .reusable = true}, SHIFT(2),
  [11] = {.count = 1, .reusable = false}, SHIFT(8),
  [13] = {.count = 1, .reusable = true}, REDUCE(sym_unit, 1),
  [15] = {.count = 1, .reusable = true}, SHIFT(3),
  [17] = {.count = 1, .reusable = true}, REDUCE(aux_sym_unit_repeat1, 2),
  [19] = {.count = 2, .reusable = false}, REDUCE(aux_sym_unit_repeat1, 2), SHIFT_REPEAT(7),
  [22] = {.count = 2, .reusable = true}, REDUCE(aux_sym_unit_repeat1, 2), SHIFT_REPEAT(12),
  [25] = {.count = 2, .reusable = true}, REDUCE(aux_sym_unit_repeat1, 2), SHIFT_REPEAT(3),
  [28] = {.count = 2, .reusable = false}, REDUCE(aux_sym_unit_repeat1, 2), SHIFT_REPEAT(8),
  [31] = {.count = 1, .reusable = true}, SHIFT(9),
  [33] = {.count = 1, .reusable = true}, SHIFT(10),
  [35] = {.count = 1, .reusable = true}, SHIFT(11),
  [37] = {.count = 1, .reusable = true}, REDUCE(sym__expression, 1),
  [39] = {.count = 1, .reusable = false}, REDUCE(sym__expression, 1),
  [41] = {.count = 1, .reusable = true}, SHIFT(5),
  [43] = {.count = 1, .reusable = true}, REDUCE(sym_bool, 1),
  [45] = {.count = 1, .reusable = false}, REDUCE(sym_bool, 1),
  [47] = {.count = 1, .reusable = true}, REDUCE(sym_lambda, 3, .production_id = 1),
  [49] = {.count = 1, .reusable = false}, REDUCE(sym_lambda, 3, .production_id = 1),
  [51] = {.count = 1, .reusable = true}, REDUCE(sym_binding, 3),
  [53] = {.count = 1, .reusable = false}, REDUCE(sym_binding, 3),
  [55] = {.count = 1, .reusable = true}, REDUCE(sym_lambda, 4, .production_id = 2),
  [57] = {.count = 1, .reusable = false}, REDUCE(sym_lambda, 4, .production_id = 2),
  [59] = {.count = 1, .reusable = true}, SHIFT(13),
  [61] = {.count = 1, .reusable = true}, SHIFT(4),
  [63] = {.count = 1, .reusable = true}, SHIFT(14),
  [65] = {.count = 1, .reusable = true}, SHIFT(6),
  [67] = {.count = 2, .reusable = true}, REDUCE(aux_sym_lambda_repeat1, 2), SHIFT_REPEAT(14),
  [70] = {.count = 1, .reusable = true}, REDUCE(aux_sym_lambda_repeat1, 2),
  [72] = {.count = 1, .reusable = true},  ACCEPT_INPUT(),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_fang(void) {
  static TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .symbol_metadata = ts_symbol_metadata,
    .parse_table = (const unsigned short *)ts_parse_table,
    .small_parse_table = (const uint16_t *)ts_small_parse_table,
    .small_parse_table_map = (const uint32_t *)ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .lex_modes = ts_lex_modes,
    .symbol_names = ts_symbol_names,
    .public_symbol_map = ts_symbol_map,
    .alias_sequences = (const TSSymbol *)ts_alias_sequences,
    .field_count = FIELD_COUNT,
    .field_names = ts_field_names,
    .field_map_slices = (const TSFieldMapSlice *)ts_field_map_slices,
    .field_map_entries = (const TSFieldMapEntry *)ts_field_map_entries,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .lex_fn = ts_lex,
    .keyword_lex_fn = ts_lex_keywords,
    .keyword_capture_token = sym_identifier,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
