# bats file_tags=tag:this
skip
@test "this/nested_closure.lox" {
  run target/debug/lox test/cases/this/nested_closure.lox

  [ "${lines[0]}" = "Foo" ]
}
