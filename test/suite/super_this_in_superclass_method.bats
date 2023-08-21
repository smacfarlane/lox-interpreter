# bats file_tags=tag:super
skip
@test "super/this_in_superclass_method.lox" {
  run target/debug/lox test/cases/super/this_in_superclass_method.lox

  [ "${lines[0]}" = "a" ]
  [ "${lines[1]}" = "b" ]
}
