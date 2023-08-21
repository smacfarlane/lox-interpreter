# bats file_tags=tag:for
@test "for/return_closure.lox" {
  run target/debug/lox test/cases/for/return_closure.lox

  [ "${lines[0]}" = "i" ]
}
