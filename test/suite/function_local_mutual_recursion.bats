# bats file_tags=tag:function
@test "function/local_mutual_recursion.lox" {
  run target/debug/lox test/cases/function/local_mutual_recursion.lox

}
