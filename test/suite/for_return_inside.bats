# bats file_tags=tag:for
@test "for/return_inside.lox" {
  run target/debug/lox test/cases/for/return_inside.lox

  [ "${lines[0]}" = "i" ]
}
