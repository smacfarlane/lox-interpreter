# bats file_tags=tag:function
skip
@test "function/recursion.lox" {
  run target/debug/lox test/cases/function/recursion.lox

  [ "${lines[0]}" = "21" ]
}
