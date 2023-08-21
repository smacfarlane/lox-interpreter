# bats file_tags=tag:operator
@test "operator/subtract.lox" {
  run target/debug/lox test/cases/operator/subtract.lox

  [ "${lines[0]}" = "1" ]
  [ "${lines[1]}" = "0" ]
}
