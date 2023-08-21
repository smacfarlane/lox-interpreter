# bats file_tags=tag:expressions
skip
@test "expressions/parse.lox" {
  run target/debug/lox test/cases/expressions/parse.lox

  [ "${lines[0]}" = "(+ (group (- 5.0 (group (- 3.0 1.0)))) (- 1.0))" ]
}
