# bats file_tags=tag:closure
@test "closure/close_over_later_variable.lox" {
  run target/debug/lox test/cases/closure/close_over_later_variable.lox

  [ "${lines[0]}" = "b" ]
  [ "${lines[1]}" = "a" ]
}
