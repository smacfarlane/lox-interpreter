# bats file_tags=tag:closure
@test "closure/open_closure_in_function.lox" {
  run target/debug/lox test/cases/closure/open_closure_in_function.lox

  [ "${lines[0]}" = "local" ]
}
