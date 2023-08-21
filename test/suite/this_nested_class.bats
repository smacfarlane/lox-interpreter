# bats file_tags=tag:this
skip
@test "this/nested_class.lox" {
  run target/debug/lox test/cases/this/nested_class.lox

  [ "${lines[0]}" = "Outer instance" ]
  [ "${lines[1]}" = "Outer instance" ]
  [ "${lines[2]}" = "Inner instance" ]
}
