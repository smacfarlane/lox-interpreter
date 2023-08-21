# bats file_tags=tag:function
skip
@test "function/local_recursion.lox" {
  run target/debug/lox test/cases/function/local_recursion.lox

  [ "${lines[0]}" = "21" ]
}
