# bats file_tags=tag:inheritance
skip
@test "inheritance/inherit_methods.lox" {
  run target/debug/lox test/cases/inheritance/inherit_methods.lox

  [ "${lines[0]}" = "foo" ]
  [ "${lines[1]}" = "bar" ]
  [ "${lines[2]}" = "bar" ]
}
