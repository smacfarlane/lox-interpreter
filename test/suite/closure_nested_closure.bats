# bats file_tags=tag:closure
skip
@test "closure/nested_closure.lox" {
  run target/debug/lox test/cases/closure/nested_closure.lox

  [ "${lines[0]}" = "a" ]
  [ "${lines[1]}" = "b" ]
  [ "${lines[2]}" = "c" ]
}
