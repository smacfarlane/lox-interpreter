# bats file_tags=tag:operator
skip
@test "operator/equals_method.lox" {
  run target/debug/lox test/cases/operator/equals_method.lox

  [ "${lines[0]}" = "true" ]
  [ "${lines[1]}" = "false" ]
}
