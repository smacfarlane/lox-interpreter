# bats file_tags=tag:operator
skip
@test "operator/not_class.lox" {
  run target/debug/lox test/cases/operator/not_class.lox

  [ "${lines[0]}" = "false" ]
  [ "${lines[1]}" = "false" ]
}
