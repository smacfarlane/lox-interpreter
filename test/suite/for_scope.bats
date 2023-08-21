# bats file_tags=tag:for
skip
@test "for/scope.lox" {
  run target/debug/lox test/cases/for/scope.lox

  [ "${lines[0]}" = "0" ]
  [ "${lines[1]}" = "-1" ]
  [ "${lines[2]}" = "after" ]
  [ "${lines[3]}" = "0" ]
}
