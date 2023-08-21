# bats file_tags=tag:this
skip
@test "this/this_in_method.lox" {
  run target/debug/lox test/cases/this/this_in_method.lox

  [ "${lines[0]}" = "baz" ]
}
