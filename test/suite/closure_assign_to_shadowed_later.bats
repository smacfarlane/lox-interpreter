# bats file_tags=tag:closure
skip
@test "closure/assign_to_shadowed_later.lox" {
  run target/debug/lox test/cases/closure/assign_to_shadowed_later.lox

  [ "${lines[0]}" = "inner" ]
  [ "${lines[1]}" = "assigned" ]
}
