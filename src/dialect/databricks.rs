// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::dialect::{Dialect, Precedence};
use crate::parser::{Parser, ParserError};
use crate::tokenizer::Token;

/// A [`Dialect`] for [Databricks SQL](https://www.databricks.com/)
///
/// See <https://docs.databricks.com/en/sql/language-manual/index.html>.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DatabricksDialect;

impl Dialect for DatabricksDialect {
    /// `<<` and `>>`, binding tighter than `&` and looser than `+`, as Spark's
    /// grammar orders them.
    fn supports_bitwise_shift_operators(&self) -> bool {
        true
    }

    fn get_next_precedence(&self, parser: &Parser) -> Option<Result<u8, ParserError>> {
        match parser.peek_token_ref().token {
            Token::ShiftLeft | Token::ShiftRight => {
                Some(Ok(self.prec_value(Precedence::Ampersand) + 1))
            }
            _ => None,
        }
    }

    // see https://docs.databricks.com/en/sql/language-manual/sql-ref-identifiers.html

    fn is_delimited_identifier_start(&self, ch: char) -> bool {
        matches!(ch, '`')
    }

    /// See <https://docs.databricks.com/en/sql/language-manual/sql-ref-identifiers.html>
    fn identifier_quote_style(&self, _identifier: &str) -> Option<char> {
        Some('`')
    }

    fn is_identifier_start(&self, ch: char) -> bool {
        matches!(ch, 'a'..='z' | 'A'..='Z' | '_')
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
    }

    fn supports_numeric_prefix(&self) -> bool {
        true
    }

    /// `'%\_%'` keeps its backslash, as Hive's `unescapeSQLString` does, so a
    /// LIKE pattern survives the literal.
    fn ignores_wildcard_escapes(&self) -> bool {
        true
    }

    /// `TABLE t` as a statement
    fn supports_table_statement(&self) -> bool {
        true
    }

    /// `VALUES (1, 2) AS t(a, b)` as a query body
    fn supports_values_alias(&self) -> bool {
        true
    }

    /// `1Y`, `1S`, `1L`, `1.0F`, `1.0D`, `1.0BD`
    fn supports_typed_numeric_literal_suffix(&self) -> bool {
        true
    }

    fn supports_filter_during_aggregation(&self) -> bool {
        true
    }

    // https://docs.databricks.com/en/sql/language-manual/sql-ref-syntax-qry-select-groupby.html
    fn supports_group_by_expr(&self) -> bool {
        true
    }

    /// <https://docs.databricks.com/gcp/en/delta/history#delta-time-travel-syntax>
    fn supports_table_versioning(&self) -> bool {
        true
    }

    fn supports_lambda_functions(&self) -> bool {
        true
    }

    // https://docs.databricks.com/en/sql/language-manual/sql-ref-syntax-qry-select.html#syntax
    fn supports_select_wildcard_except(&self) -> bool {
        true
    }

    /// The unit after an interval value is optional. Databricks documents the
    /// qualified form, `INTERVAL '3' DAY`; Databricks Runtime parses with
    /// Spark's grammar, whose multi-units syntax also lets the string carry its
    /// own units, `INTERVAL '1 YEAR 2 DAYS 3 HOURS'`. An interval value is always
    /// a literal, never an expression.
    ///
    /// See <https://docs.databricks.com/aws/en/sql/language-manual/data-types/interval-type>
    /// and <https://spark.apache.org/docs/latest/sql-ref-literals.html#interval-literal>
    fn require_interval_qualifier(&self) -> bool {
        false
    }

    // See https://docs.databricks.com/en/sql/language-manual/functions/struct.html
    fn supports_struct_literal(&self) -> bool {
        true
    }

    /// See <https://docs.databricks.com/aws/en/sql/language-manual/sql-ref-syntax-comment>
    fn supports_nested_comments(&self) -> bool {
        true
    }

    /// See <https://docs.databricks.com/en/sql/language-manual/sql-ref-syntax-qry-select-groupby.html>
    fn supports_group_by_with_modifier(&self) -> bool {
        true
    }

    /// See <https://docs.databricks.com/en/sql/language-manual/sql-ref-syntax-qry-select-values.html>
    fn supports_values_as_table_factor(&self) -> bool {
        true
    }

    /// See <https://docs.databricks.com/en/sql/language-manual/delta-optimize.html>
    fn supports_optimize_table(&self) -> bool {
        true
    }

    /// See <https://docs.databricks.com/aws/en/sql/language-manual/functions/bangsign>
    fn supports_bang_not_operator(&self) -> bool {
        true
    }

    /// See <https://docs.databricks.com/aws/en/sql/language-manual/sql-ref-syntax-qry-select-cte>
    fn supports_cte_without_as(&self) -> bool {
        true
    }

    fn supports_select_item_multi_column_alias(&self) -> bool {
        true
    }

    fn supports_map_literal_with_angle_brackets(&self) -> bool {
        true
    }

    fn supports_string_literal_backslash_escape(&self) -> bool {
        true
    }

    fn supports_select_wildcard_replace(&self) -> bool {
        true
    }

    fn supports_from_first_select(&self) -> bool {
        true
    }

    fn supports_pipe_operator(&self) -> bool {
        true
    }
}
