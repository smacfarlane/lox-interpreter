# bats file_tags=tag:operator
@test "operator/add.lox" {
  run target/debug/lox test/cases/operator/add.lox

  [ "${lines[0]}" = "579" ]
  [ "${lines[1]}" = "string" ]
}
