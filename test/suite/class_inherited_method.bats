# bats file_tags=tag:class
skip
@test "class/inherited_method.lox" {
  run target/debug/lox test/cases/class/inherited_method.lox

  [ "${lines[0]}" = "in foo" ]
  [ "${lines[1]}" = "in bar" ]
  [ "${lines[2]}" = "in baz" ]
}
