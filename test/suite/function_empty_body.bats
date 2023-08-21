# bats file_tags=tag:function
@test "function/empty_body.lox" {
  run target/debug/lox test/cases/function/empty_body.lox

  [ "${lines[0]}" = "nil" ]
}
