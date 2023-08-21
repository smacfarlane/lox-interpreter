# bats file_tags=tag:function
skip
@test "function/print.lox" {
  run target/debug/lox test/cases/function/print.lox

  [ "${lines[0]}" = "<fn foo>" ]
  [ "${lines[1]}" = "<native fn>" ]
}
