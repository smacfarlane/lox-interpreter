# bats file_tags=tag:function
skip
@test "function/mutual_recursion.lox" {
  run target/debug/lox test/cases/function/mutual_recursion.lox

  [ "${lines[0]}" = "true" ]
  [ "${lines[1]}" = "true" ]
}
