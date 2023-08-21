# bats file_tags=tag:operator
@test "operator/multiply.lox" {
  run target/debug/lox test/cases/operator/multiply.lox

  [ "${lines[0]}" = "15" ]
  [ "${lines[1]}" = "3.702" ]
}
