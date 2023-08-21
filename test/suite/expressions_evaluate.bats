# bats file_tags=tag:expressions
skip
@test "expressions/evaluate.lox" {
  run target/debug/lox test/cases/expressions/evaluate.lox

  [ "${lines[0]}" = "2" ]
}
