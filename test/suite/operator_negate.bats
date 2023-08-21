# bats file_tags=tag:operator
@test "operator/negate.lox" {
  run target/debug/lox test/cases/operator/negate.lox

  [ "${lines[0]}" = "-3" ]
  [ "${lines[1]}" = "3" ]
  [ "${lines[2]}" = "-3" ]
}
