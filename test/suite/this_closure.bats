# bats file_tags=tag:this
skip
@test "this/closure.lox" {
  run target/debug/lox test/cases/this/closure.lox

  [ "${lines[0]}" = "Foo" ]
}
