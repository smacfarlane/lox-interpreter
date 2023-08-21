# bats file_tags=tag:operator
@test "operator/divide.lox" {
  run target/debug/lox test/cases/operator/divide.lox

  [ "${lines[0]}" = "4" ]
  [ "${lines[1]}" = "1" ]
}
