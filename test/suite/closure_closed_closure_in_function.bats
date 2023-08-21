# bats file_tags=tag:closure
@test "closure/closed_closure_in_function.lox" {
  run target/debug/lox test/cases/closure/closed_closure_in_function.lox

  [ "${lines[0]}" = "local" ]
}
