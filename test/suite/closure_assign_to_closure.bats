# bats file_tags=tag:closure
skip
@test "closure/assign_to_closure.lox" {
  run target/debug/lox test/cases/closure/assign_to_closure.lox

  [ "${lines[0]}" = "local" ]
  [ "${lines[1]}" = "after f" ]
  [ "${lines[2]}" = "after f" ]
  [ "${lines[3]}" = "after g" ]
}
