# bats file_tags=tag:closure
@test "closure/reference_closure_multiple_times.lox" {
  run target/debug/lox test/cases/closure/reference_closure_multiple_times.lox

  [ "${lines[0]}" = "a" ]
  [ "${lines[1]}" = "a" ]
}
