# bats file_tags=tag:function
@test "function/nested_call_with_arguments.lox" {
  run target/debug/lox test/cases/function/nested_call_with_arguments.lox

  [ "${lines[0]}" = "hello world" ]
}
